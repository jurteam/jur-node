#!/bin/bash

if [ "$(docker ps -a -q)" ]; then
  docker stop $(docker ps -a -q)  # stop all containers
  docker rm $(docker ps -a -q)  # remove all containers
fi

AURA_KEY="${KEY_PREFIX}_AURA_KEY"
GRANPA_KEY="${KEY_PREFIX}_GRANPA_KEY"

echo "key" 
echo $$AURA_KEY

sudo echo AURA_KEY=$$AURA_KEY >> /etc/environment
sudo echo GRANPA_KEY=$$GRANPA_KEY >> /etc/environment


gcloud --quiet auth activate-service-account --key-file=key.json && \
gcloud --quiet auth configure-docker && \

chmod +x ./start-jur-node.sh && \

docker image prune -a -f && \
docker-compose -f docker-compose-$NETWORK_TYPE.yml up -d && \
echo "[$(date)] Successfully deployed" >> deploy.log
