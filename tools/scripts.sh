#! /usr/bin/bash

rootd() {
  pushd $(git rev-parse --show-toplevel) 2>&1 > /dev/null
}

restored() {
  popd 2>&1 > /dev/null
}

get_parameter() {
  DEFAULT="${1}"
  if [ -z ${2} ]; then
    echo "${DEFAULT}"
  elif [ ! -d "${2}" ]; then
    echo "${DEFAULT}"
  else
    echo "${2}"
  fi
}

# Usage:   sol_chess_build_web  debug|release   target_dir               [archive_dir]
#                               Build profile   location to place files  location to place compressed archive
sol_chess_build_web() {
  rootd

  local BUILD_PROFILE="debug"
  local BUILD_PROFILE_SWITCH=""
  if [ -n "${1}" ]; then
    if [ "${1}" = "release" ]; then
      local BUILD_PROFILE="release"
      local BUILD_PROFILE_SWITCH="--release"
    fi
  fi

  local TARGET_DIR=$(get_parameter "./target/dist" ${2})
  local ARCHIVE_DIR=$(get_parameter "" ${3})

  echo "Build profile: ${BUILD_PROFILE}"
  echo "Build profile switch: ${BUILD_PROFILE_SWITCH}"
  echo "Target directory: ${TARGET_DIR}"
  echo "Archive directory: ${ARCHIVE_DIR}"

  set -x
  cargo build --target wasm32-unknown-unknown ${BUILD_PROFILE_SWITCH}
  set +x
  if [ $? -ne 0 ]; then
      echo "Wasm build failed"
      return 1
  fi

  rm -rf ${TARGET_DIR} && mkdir -p ${TARGET_DIR} && mv ./target/wasm32-unknown-unknown/${BUILD_PROFILE}/sol_chess.wasm ${TARGET_DIR}/sol_chess.wasm && cp ./tools/web/index.html ${TARGET_DIR}/index.html
  if [ $? -ne 0 ]; then
      echo "Failed to assemble the build in ${TARGET_DIR}"
      return 1
  fi

  if [ -n "${ARCHIVE_DIR}" ]; then
    local TAR_NAME="${ARCHIVE_DIR}/sol_chess.tar.gz"
    set -x
    tar -czvf ${TAR_NAME} -C ${TARGET_DIR} . && echo "Created ${TAR_NAME}"
    set +x
  fi

  restored
}

sol_chess_web_local() {
  rootd

  local TARGET_DIR=$(get_parameter "./target/dist" ${1})
  echo "Building web app in ${TARGET_DIR}"
  sol_chess_build_web "debug" $TARGET_DIR
  if [ $? -ne 0 ]; then
      echo "Failed to build the web app"
      return 1
  fi

  basic-http-server $TARGET_DIR

  restored
}

sol_chess_dev() {
  rootd

  TESTING=1 cargo run

  restored
}

sol_chess_deploy() {
  rootd

  if [ $# -ne 1 ]; then
    echo "Usage: sol_chess_deploy <serve_root>"
    return 1
  fi

  if [ ! -d $1 ]; then
    echo "Directory $1 does not exist"
    return 1
  fi

  local serve_root=$1
  sol_chess_build_web "release" "./target/dist" "./target"
  if [ $? -ne 0 ]; then
      echo "Failed to build the web app"
      return 1
  fi

  sudo mv ./target/sol_chess.tar.gz $serve_root/sol_chess.tar.gz && \
  sudo tar -xzvf $serve_root/sol_chess.tar.gz -C $serve_root && \
  sudo rm $serve_root/sol_chess.tar.gz
  echo "Deployment complete"

  restored
}

sol_chess_clean() {
  rootd

  rm -rf ./target

  restored
}
