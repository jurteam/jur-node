#!/bin/bash

DEPLOY_PATH=$HOME/jur-node # NOTE: change this to your codebase location
GIT_REGISTRY="git@github.com:jurteam/jur-node.git"
DOCKER_REGISTRY="registry.digitalocean.com"
DC_FILE="$DEPLOY_PATH/docker-compose-prod.yml"

DOCKER_NOT_FOUND=21
DOCKER_COMPOSE_NOT_FOUND=22
PARAMETER_MISSING=23

if [[ ! $(which git) ]]; then
  echo "[$(date)] No git found" >> deploy.log
  sudo apt update \
  sudo apt install git
  echo "[$(date)] git installed" >> deploy.log
fi

gitBranch() {
  git rev-parse --abbrev-ref HEAD
}

gitCommit() {
  git rev-parse --short HEAD
}

if [ ! -f "$DEPLOY_PATH/.circleci/config.yml" ]; then
  echo "[$(date)] Cloning repo to $DEPLOY_PATH" >> deploy.log
  git clone $GIT_REGISTRY $DEPLOY_PATH
fi

if [[ ! $(which docker) ]]; then
  echo "[$(date)] No docker found" >> deploy.log
  removeOld=$(sudo apt-get remove docker docker-engine docker.io containerd runc)
  echo "[$(date)] Output of removing old docker packages: $removeOld" >> deploy.log
  sudo apt-get update
  sudo apt-get install \
    apt-transport-https \
    ca-certificates \
    curl \
    gnupg-agent \
    software-properties-common
  curl -fsSL https://download.docker.com/linux/ubuntu/gpg | sudo apt-key add -
  dockerKey=$(sudo apt-key fingerprint 0EBFCD88)
  echo "[$(date)] Docker fingerprint: $dockerKey" >> deploy.log

  sudo add-apt-repository \
   "deb [arch=amd64] https://download.docker.com/linux/ubuntu \
   $(lsb_release -cs) \
   stable"

  sudo apt-get update
  sudo apt-get install -y docker-ce docker-ce-cli containerd.io

  echo "[$(date)] Docker after installation steps $(which docker)" >> deploy.log
  if [[ ! $(which docker) ]]; then
    exit $DOCKER_NOT_FOUND
  fi
fi

if [[ ! $(which docker-compose) ]]; then
  echo "[$(date)] No docker-compose found" >> deploy.log
  sudo curl -L "https://github.com/docker/compose/releases/download/1.27.4/docker-compose-$(uname -s)-$(uname -m)" -o /usr/local/bin/docker-compose
  sudo chmod +x /usr/local/bin/docker-compose

  echo "[$(date)] Docker Compose after installation steps $(which docker-compose)" >> deploy.log
  if [[ ! $(which docker-compose) ]]; then
    exit $DOCKER_COMPOSE_NOT_FOUND
  fi
fi

if [ -z $1 ]; then
  echo "Needs exactly three arg. Please provide docker login credentials"
  exit $PARAMETER_MISSING
fi

if [ -z $2 ]; then
  echo "Needs exactly three arg. Please provide aura key credentials"
  exit $PARAMETER_MISSING
fi

if [ -z $3 ]; then
  echo "Needs exactly three arg. Please provide granpa key credentials"
  exit $PARAMETER_MISSING
fi

pushd $DEPLOY_PATH && \
echo "[$(date)] Starting deploy $(gitBranch) @ $(gitCommit)" >> deploy.log && \
git fetch -p && \
git pull && \
echo "[$(date) Updated git to $(gitBranch) @ $(gitCommit)]" >> deploy.log && \
docker login -u $1 -p $1 $DOCKER_REGISTRY && \
docker-compose -f $DC_FILE pull && \
docker-compose -f $DC_FILE stop && \
docker-compose -f $DC_FILE up -d && \
docker exec jur_node_container jur-node key insert --chain jur-testnet --scheme Sr25519 --suri $2 --key-type aura && \
docker exec jur_node_container jur-node key insert --chain jur-testnet --scheme Ed25519 --suri $3 --key-type gran && \
echo docker exec jur_node_container jur-node key insert --chain jur-testnet --scheme Sr25519 --suri $2 --key-type aura >> deploy.log && \
echo docker exec jur_node_container jur-node key insert --chain jur-testnet --scheme Ed25519 --suri $3 --key-type gran >> deploy.log && \
echo "[$(date)] Successfully deployed" >> deploy.log && \
popd