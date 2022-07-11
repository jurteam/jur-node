#!/bin/bash

ARGUMENT_MISSING=23

if [ -z $1 ]; then
  echo "Needs exactly two arg. Please provide Aura secret"
  exit $ARGUMENT_MISSING
fi

if [ -z $2 ]; then
  echo "Needs exactly two arg. Please provide Granpa secret"
  exit $ARGUMENT_MISSING
fi

docker exec jur_node_container jur-node key insert --chain jur-testnet --scheme Sr25519 --suri $1 --key-type aura

docker exec jur_node_container jur-node key insert --chain jur-testnet --scheme Ed25519 --suri $2 --key-type gran
