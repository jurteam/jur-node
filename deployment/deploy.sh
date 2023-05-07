#!/bin/bash

DEPLOY_PATH=$HOME/deployment # NOTE: change this to your codebase location

if [ "$(docker ps -a -q)" ]; then
  docker stop $(docker ps -a -q)  # stop all containers
  docker rm $(docker ps -a -q)  # remove all containers
fi

sudo echo AURA_KEY="${${KEY_PREFIX}_AURA_KEY}" >> /etc/environment
sudo echo GRANPA_KEY="${${KEY_PREFIX}_GRANPA_KEY}" >> /etc/environment

pushd $DEPLOY_PATH && \
gcloud --quiet auth activate-service-account --key-file=key.json && \
gcloud --quiet auth configure-docker && \

chmod +x ./start-jur-node.sh && \

docker image prune -a -f && \
docker-compose -f docker-compose-$NETWORK_TYPE.yml up -d && \
echo "[$(date)] Successfully deployed" >> deploy.log && \
popd