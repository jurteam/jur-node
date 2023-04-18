#!/bin/bash

DEPLOY_PATH=$HOME # NOTE: change this to your codebase location
DOCKER_REGISTRY="us.gcr.io"
IMAGE_NAME="$DOCKER_REGISTRY/layer1-mvp/jur-node:latest"

PARAMETER_MISSING=23

if [ -z $1 ]; then
  echo "Needs exactly two arg. Please provide aura key credentials"
  exit $PARAMETER_MISSING
fi

if [ -z $2 ]; then
  echo "Needs exactly two arg. Please provide granpa key credentials"
  exit $PARAMETER_MISSING
fi

if [ $(docker ps -a -q) ]; then
  docker stop $(docker ps -a -q)  # stop all containers
  docker rm $(docker ps -a -q)  # remove all containers
fi

pushd $DEPLOY_PATH && \
gcloud auth activate-service-account --key-file=key.json
gcloud auth configure-docker

docker image prune -a -f && \
docker pull $IMAGE_NAME && \
docker run -d -v data:/data -p 30333:30333 -p 9933:9933 -p 9944:9944 -p 9615:9615 --name=jur_node_container $IMAGE_NAME jur-node --chain jur-testnet --port 30333 --ws-port 9944 --rpc-port 9933 --validator 
docker exec jur_node_container jur-node key insert --chain jur-testnet --scheme Sr25519 --suri $1 --key-type aura && \
docker exec jur_node_container jur-node key insert --chain jur-testnet --scheme Ed25519 --suri $2 --key-type gran && \
echo "[$(date)] Successfully deployed" >> deploy.log && \
popd