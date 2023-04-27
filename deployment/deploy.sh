#!/bin/bash

DEPLOY_PATH=$HOME/deployment # NOTE: change this to your codebase location

if [ "$(docker ps -a -q)" ]; then
  docker stop $(docker ps -a -q)  # stop all containers
  docker rm $(docker ps -a -q)  # remove all containers
fi

pushd $DEPLOY_PATH && \
gcloud --quiet auth activate-service-account --key-file=key.json
gcloud --quiet auth configure-docker

docker image prune -a -f && \
docker-compose -f docker-compose-testnet.yml up -d && \
echo "[$(date)] Successfully deployed" >> deploy.log && \
popd