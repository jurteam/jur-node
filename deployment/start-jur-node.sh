#!/bin/bash

PORT=30333
RPC_PORT=9944
SPEC_FILE_PATH="./spec.json"
BOOT_NODE_ID=$(head -n 1 ./bootnode_id.txt)

jur-node key insert --base-path  /tmp/node --chain $SPEC_FILE_PATH --scheme Sr25519 --suri $AURA_KEY --key-type aura
jur-node key insert --base-path  /tmp/node --chain $SPEC_FILE_PATH --scheme Ed25519 --suri $GRANPA_KEY --key-type gran

if [ "${BOOT_NODE}" == "TRUE" ]; then
    jur-node --base-path /tmp/node --chain $SPEC_FILE_PATH --port $PORT --rpc-port $RPC_PORT --validator --blocks-pruning archive --state-pruning archive --rpc-max-connections 300 --unsafe-rpc-external --rpc-cors all
else
    jur-node --base-path /tmp/node --chain $SPEC_FILE_PATH --port $PORT --validator --unsafe-rpc-external --rpc-cors all --bootnodes /ip4/$BOOT_NODE_IP/tcp/30333/p2p/$BOOT_NODE_ID
fi