#!/bin/bash

DEPLOY_PATH=$HOME # NOTE: change this to your codebase location
DOCKER_REGISTRY="registry.digitalocean.com"
IMAGE_NAME="$DOCKER_REGISTRY/jur-jbp/jur-node"

DOCKER_NOT_FOUND=21
DOCKER_COMPOSE_NOT_FOUND=22
PARAMETER_MISSING=23


if [[ ! $(which docker) ]]; then
  echo "[$(date)] No docker found" >> deploy.log
  removeOld=$(sudo apt-get remove docker docker-engine docker.io containerd runc)
  echo "[$(date)] Output of removing old docker packages: $removeOld" >> deploy.log
  ### Docker and docker compose prerequisites
  sudo apt-get -y install curl gnupg ca-certificates lsb-release

  sudo apt-get install -y docker.io

  sudo groupadd docker

  sudo usermod -aG docker ${USER}

  sudo chmod 666 /var/run/docker.sock

  echo "[$(date)] Docker after installation steps $(which docker)" >> deploy.log
  if [[ ! $(which docker) ]]; then
    exit $DOCKER_NOT_FOUND
  fi
fi


if [ -z $1 ]; then
  echo "Needs exactly four arg. Please provide docker login credentials"
  exit $PARAMETER_MISSING
fi

if [ -z $2 ]; then
  echo "Needs exactly four arg. Please provide aura key credentials"
  exit $PARAMETER_MISSING
fi

if [ -z $3 ]; then
  echo "Needs exactly four arg. Please provide granpa key credentials"
  exit $PARAMETER_MISSING
fi

if [ -z $4 ]; then
  echo "Needs exactly four arg. Please provide docker tag"
  exit $PARAMETER_MISSING
fi

docker login -u $1 -p $1 $DOCKER_REGISTRY 
if [ $(docker ps -a -q) ]; then
  docker stop $(docker ps -a -q)  # stop all containers
  docker rm $(docker ps -a -q)  # remove all containers
fi

pushd $DEPLOY_PATH && \
docker image prune -a -f && \
docker pull $IMAGE_NAME:$4 && \
docker run -d -v data:/data -p 30333:30333 -p 9933:9933 -p 9944:9944 -p 9615:9615 --name=jur_node_container $IMAGE_NAME:$4 jur-node --chain jur-testnet --port 30333 --ws-port 9944 --rpc-port 9933 --validator --bootnodes /ip4/34.171.35.134/tcp/30333/p2p/12D3KooWBBb45AWo5cnFhurPav2aSgpsPewPfLmEdCW8Xw7wiuAF && \
docker exec jur_node_container jur-node key insert --chain jur-testnet --scheme Sr25519 --suri $2 --key-type aura && \
docker exec jur_node_container jur-node key insert --chain jur-testnet --scheme Ed25519 --suri $3 --key-type gran && \
echo "[$(date)] Successfully deployed" >> deploy.log && \
popd