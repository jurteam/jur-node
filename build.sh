#!/bin/bash

showHelp() {
cat <<- EOL

Usage:

build.sh without any arg will only build a docker image with non-conflicting image tag
-h  Print this message
-p  Push only (image must already exist)
-a  Build and then push
EOL
}

REGISTRY="registry.digitalocean.com/jur-jbp"
JUR_NODE="jur-node"

# Example Tag="08OCT2022"
TAG=$(date +%d%b%Y | tr a-z A-Z)

IMAGE_NAME_JUR_NODE="$REGISTRY/$JUR_NODE"
IMAGE_JUR_NODE="$IMAGE_NAME_JUR_NODE:$TAG"

shouldBuild=true
shouldPush=false

while getopts ":hpa" opt; do
case $opt in
  h ) showHelp; exit 0 ;;
  p ) shouldPush=true; shouldBuild=false ;;
  a ) shouldBuild=true; shouldPush=true;;
  \? ) echo "Unknown arg"; showHelp; exit 1 ;;
esac
done

arch="$(uname -p)"

if [ $arch = "arm" ]; then
    echo "🦋 Doesn't work on m1"
    echo "Build manually using: docker build -f ./Dockerfile -t registry.digitalocean.com/jur-jbp/jur-node:<date-tag>-arm64 ./"
    exit 1;
fi

if [ $shouldBuild == true ]; then
  existingImagesCount=$(docker images ${IMAGE_NAME_JUR_NODE} | grep ${TAG} | wc -l)
  while [ $existingImagesCount -ne 0 ]; do
    echo "An image already exists with the same name and tag"
    TAG="$TAG-$((existingImagesCount + 1))"
    echo "Will use updated tag: $TAG"
    IMAGE_JUR_NODE="$IMAGE_NAME_JUR_NODE:$TAG"
    existingImagesCount=$(docker images ${IMAGE_NAME_JUR_NODE} | grep ${TAG} | wc -l)
  done

  echo "Building image $IMAGE_JUR_NODE"

  sed -i "s/$JUR_NODE:[A-Z0-9\-]*/$JUR_NODE:$TAG/g" docker-compose-prod.yml && \
  sed -i "s/$JUR_NODE:[A-Z0-9\-]*/$JUR_NODE:$TAG/g" .circleci/config.yml && \
  docker build -f ./Dockerfile -t $IMAGE_JUR_NODE ./
fi

if [ $shouldPush == true ]; then
  echo "Pushing image $IMAGE_JUR_NODE"
  docker push $IMAGE_JUR_NODE
fi