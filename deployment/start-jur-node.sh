#!/bin/bash

PORT=30333
WS_PORT=9944
RPC_PORT=9933
SPEC_FILE_PATH="./spec.json"
BOOT_NODE_ID=$(head -n 1 ./bootnode_id.txt)

echo "bootnode id is $BOOT_NODE_ID"

jur-node key insert --base-path  /tmp/node --chain $SPEC_FILE_PATH --scheme Sr25519 --suri $AURA_KEY --key-type aura
jur-node key insert --base-path  /tmp/node --chain $SPEC_FILE_PATH --scheme Ed25519 --suri $GRANPA_KEY --key-type gran

if [ "${BOOT_NODE}" == "TRUE" ]; then
    jur-node --base-path /tmp/node --chain $SPEC_FILE_PATH --port $PORT --ws-port $WS_PORT --rpc-port $RPC_PORT --validator --blocks-pruning archive --state-pruning archive --ws-max-connections 100 --unsafe-ws-external --rpc-cors all
else
    jur-node --base-path /tmp/node --chain $SPEC_FILE_PATH --port $PORT --validator --unsafe-ws-external --rpc-cors all --bootnodes /ip4/$BOOT_NODE_IP/tcp/30333/p2p/$BOOT_NODE_ID
fi