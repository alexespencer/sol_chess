cargo build --target wasm32-unknown-unknown --release
mkdir -p ./dist
rm ./web/sol_chess.wasm
mv ./target/wasm32-unknown-unknown/release/sol_chess.wasm ./dist/sol_chess.wasm

basic-http-server ./dist
