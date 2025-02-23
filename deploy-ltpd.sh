./build-web.sh

serve_root=$1

sudo mv ./sol_chess.tar.gz $serve_root/sol_chess.tar.gz && \
sudo tar -xzvf $serve_root/sol_chess.tar.gz -C $serve_root && \
sudo rm $serve_root/sol_chess.tar.gz

echo "Deployment complete"

