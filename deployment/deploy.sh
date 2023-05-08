#!/bin/bash

DEPLOY_PATH=/home/circleci/deployment # NOTE: change this to your codebase location
$EMPTY_ARGUMENT_ERROR_CODE=1

if [ "$(docker ps -a -q)" ]; then
  docker stop $(docker ps -a -q)  # stop all containers
  docker rm $(docker ps -a -q)  # remove all containers
fi

if [ -z $1 ] || [ -z $2 ] || [ -z $3 ] || [ -z $4 ] || [ -z $5 ] || [ -z $6 ]; then
  echo "Needs exactly six arguments. Please provide Aura and Granpa keys for all instances"
  exit $EMPTY_ARGUMENT_ERROR_CODE
fi

if [ $KEY_PREFIX == "INSTANCE_1"]; then
  sudo echo AURA_KEY=$1 >> /etc/environment
  sudo echo GRANPA_KEY=$2 >> /etc/environment
elif [ $KEY_PREFIX == "INSTANCE_2"]; then
  sudo echo AURA_KEY=$3 >> /etc/environment
  sudo echo GRANPA_KEY=$4 >> /etc/environment
elif [ $KEY_PREFIX == "INSTANCE_3"]; then
  sudo echo AURA_KEY=$5 >> /etc/environment
  sudo echo GRANPA_KEY=$6 >> /etc/environment
else
  echo "Invalid KEY_PREFIX. Please provide a valid KEY_PREFIX"
  exit $EMPTY_ARGUMENT_ERROR_CODE
fi

pushd $DEPLOY_PATH && \
gcloud --quiet auth activate-service-account --key-file=key.json && \
gcloud --quiet auth configure-docker && \

chmod +x ./start-jur-node.sh && \

docker image prune -a -f && \
docker-compose -f docker-compose-$NETWORK_TYPE.yml up -d && \
echo "[$(date)] Successfully deployed" >> deploy.log && \
popd