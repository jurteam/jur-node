#!/bin/bash

DEPLOY_PATH=/home/circleci/deployment # NOTE: change this to your codebase location

if [ "$(docker ps -a -q)" ]; then
  docker stop $(docker ps -a -q)  # stop all containers
  docker rm $(docker ps -a -q)  # remove all containers
fi

AURA_KEY_NAME="${KEY_PREFIX}_AURA_KEY"
GRANPA_KEY_NAME="${KEY_PREFIX}_GRANPA_KEY"

AURA_KEY_VALUE="${!AURA_KEY_NAME}"
GRANPA_KEY_VALUE="${!GRANPA_KEY_NAME}"

sudo echo AURA_KEY=$AURA_KEY_VALUE >> /etc/environment
sudo echo GRANPA_KEY=$GRANPA_KEY_VALUE >> /etc/environment

pushd $DEPLOY_PATH && \
gcloud --quiet auth activate-service-account --key-file=key.json && \
gcloud --quiet auth configure-docker && \

chmod +x ./start-jur-node.sh && \

docker image prune -a -f && \
docker-compose -f docker-compose-$NETWORK_TYPE.yml up -d && \
echo "[$(date)] Successfully deployed" >> deploy.log && \
popd