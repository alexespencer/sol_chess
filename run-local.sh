./build-web.sh && \
mkdir -p ./local-deploy && \
./deploy-ltpd.sh ./local-deploy && \
basic-http-server ./local-deploy
