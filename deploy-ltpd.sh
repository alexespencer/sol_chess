./build-web.sh

serve_root=$1

mv ./sol_chess.tar.gz $serve_root/sol_chess.tar.gz && \
tar -xzvf $serve_root/sol_chess.tar.gz -C $serve_root && \
rm $serve_root/sol_chess.tar.gz

echo "Deployment complete"

