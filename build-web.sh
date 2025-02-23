cargo build --target wasm32-unknown-unknown --release

if [ $? -ne 0 ]; then
    echo "Wasm build failed"
    exit 1
fi

rm -rf ./dist && mkdir -p ./dist && mv ./target/wasm32-unknown-unknown/release/sol_chess.wasm ./dist/sol_chess.wasm && cp ./web/index.html ./dist/index.html

if [ $? -ne 0 ]; then
    echo "Failed to create create dist directory"
    exit 1
fi

tar -czvf ./sol_chess.tar.gz -C ./dist . && rm -rf ./dist && echo "Web build complete"
