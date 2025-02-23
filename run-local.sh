cargo build --target wasm32-unknown-unknown --release

if [ $? -ne 0 ]; then
    echo "Wasm build failed"
    exit 1
fi

mkdir -p ./local-deploy && \
cp ./web/index.html ./local-deploy/index.html && \
cp ./target/wasm32-unknown-unknown/release/sol_chess.wasm ./local-deploy/sol_chess.wasm && \
basic-http-server ./local-deploy
