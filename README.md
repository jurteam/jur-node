<p align="center">
    <img src="https://github-production-user-asset-6210df.s3.amazonaws.com/4173518/297033697-a84bf93d-4c7b-40d7-a5e2-e107f39f55a0.png" alt="JurCahin Logo" />
</p>
<h1 align="center">Jur Node</h1>

<p align="center">Empowering Startup Societies, Countries, and Network States with on-chain and transparent governance.</p>

---

Discover the strength of Jur's unique Layer-1 blockchain, meticulously crafted with [Substrate technology](https://substrate.io/) ensuring a robust ecosystem for the future of decentralized governance. Join us in shaping a world where governance is dynamic, inclusive, and built to serve the needs of all.

# 🏆 Requirements

- Prepare your development environment ([Instructions](https://docs.substrate.io/install/))
- Clone the repository:

```
git clone git@github.com:jurteam/jur-node.git
```

- Don't forget to source cargo env file:

```
source ~/.cargo/env
```

# ⛳ Getting Started

Use this **QuickStart** command to build and launch the node:

```bash
cargo run --release -- --dev
```

<img src="https://github.com/jurteam/jur-node/assets/4173518/d9a658d9-9588-4aa9-add0-9f281f9df434">

By running the above command, all the necessary components will be pulled and the Jur node will be started in development mode.

### List of all commands

The following command can be used to explore all parameters and subcommands:

```
./target/release/jur-node -h
```

# 💻 <a name="dev-ecosystem">Development Ecosystem (Single Node)</a>

The provided `cargo run` command will launch a temporary node and its state will be discarded after you terminate the process. Use the following command to build the node without launching it:

```
cargo build --release
```

After the project has been built, you can see the binary in the location `./target/release/jur-node`.

---

The following command will start the single-node development chain with non-persistent state:

```
./target/release/jur-node --dev
```

Start the development chain with detailed logging:

```
RUST_BACKTRACE=1 ./target/release/jur-node -ldebug --dev
```

---

In order to maintain the state of the chain between runs, it is necessary to specify a base path where the database can be stored persistently instead of utilizing a temporary location. This allows for the organized storage of different chain databases, with a separate folder being created for each distinct chain run:

```sh
// Create a folder to use as the db base path
$ mkdir data

// Use of that folder to store the chain state
$ ./target/release/jur-node --dev --base-path ./data
```

# 💡 <a name="testnet-ecosystem">Testnet Ecosystem (Multi-Node)</a>

The multi-node testnet environment provides a playground for developers to experiment with Jur Chain features in a real scenario.

In order to set up the testnet, it is important to understand some key concepts. The following table summarizes the most common concepts and their descriptions:

| Concept | What it does |
| --- | --- |
| Validator node | Processes the validating proofs to finalize a block |
| Boot node | Provides a static address and peer-to-peer (libp2p) public key that is used to bootstrap a node onto the network’s distributed hash table and to find peer nodes. |
| RPC node | Exposes an RPC interface over HTTP or WebSocket ports, so that users can read the blockchain state and submit transactions. There are often multiple RPC nodes behind a load balancer. |
| Archive node | Maintains all blocks starting from the genesis block with complete state available for every block. |
| Full node | Synchronizes with the chain to store the most recent block state and block headers for older blocks. |
| Aura Key | In the Aura consensus mechanism, a set of validators take turns proposing new blocks in a deterministic order. Each validator has an associated Aura key, and the validator whose turn it is to propose a block uses their Aura key to sign the block. This ensures a predictable and secure block creation process, as validators take turns being the leader. |
| Grandpa Key | Grandpa is responsible for finalizing blocks and ensuring the overall security of the blockchain. The Grandpa mechanism enhances the security and reliability of the blockchain by ensuring that once a block is finalized, it cannot be reverted. |

Proof of Authority (PoA) is a consensus algorithm used in the Jur Node to validate and confirm transactions. In a PoA system, network participants, known as validators or nodes, are pre-approved and identified entities with recognized authority or reputation.

We need to generate Aura and Grandpa keys to set up or join a multi-node Jur Chain. Here are the commands to generate both keys:

**Generate Aura Key:**

```
./target/release/jur-node key generate --scheme Sr25519 --password-interactive -w 24
```

**Generate Grandpa Key:**

```
./target/release/jur-node key generate --scheme Ed25519 --password-interactive -w 24
```

Note: **Store both Aura and Granpa secret phrases in a safe place.**

Alternatively, one could reuse the secret phrase generated earlier and derive a new key using the Ed25519 scheme. The following command does exactly that:

```bash
./target/release/jur-node key inspect --password-interactive --scheme Ed25519 "escape gift blossom cake produce human copper rain hope embark search solid youth cricket sort dad shed december winter involve dolphin click annual liar"
```

In Proof of Authority consensus mechanism, validator keys should be specified in [chain_spec](https://github.com/jurteam/jur-node/blob/develop/res/localSpecRaw.json) ([read more](https://docs.substrate.io/tutorials/build-a-blockchain/add-trusted-nodes/)):

**Aura:**

```json
"aura": { "authorities": [
   "5CfBuoHDvZ4fd8jkLQicNL8tgjnK8pVG9AiuJrsNrRAx6CNW",
 ]
},
```

**Grandpa:**

```json
"grandpa": {
   "authorities": [
     [
       "5CuqCGfwqhjGzSqz5mnq36tMe651mU9Ji8xQ4JRuUTvPcjVN",
       1
     ]
   ]
 },
```

We need to add the keys to each validator nodes using the command:

```bash
./target/release/jur-node key insert --base-path  ./data/ \
--chain ./res/localSpecRaw.json \
--scheme Sr25519 \
--suri "<replace-aura-secret-seed>" \
--key-type aura


./target/release/jur-node key insert --base-path  ./data/ \
--chain ./res/localSpecRaw.json \
--scheme Ed25519 \
--suri "<replace-granpa-secret-seed>" \
--key-type gran
```

📝 Note: We need to run at least three nodes in order to start producing and finalizing blocks.

Here is the command to start a node, Run the command in three different terminals by replacing the dynamic fields:

```bash
./target/release/jur-node \
  --base-path "<replace-data-path>" \
  --chain ./res/localSpecRaw.json \
  --port "<replace-p2p-port>" \
  --ws-port "<replace-ws-port>" \
  --rpc-port "<replace-rpc-port>" \
  --validator \
  --rpc-methods Unsafe \
  --name "<replace-node-name>"
```

We need to specify an additional bootnode parameter on second and third nodes:

```
 --bootnodes "<replace-boot-node-id>"
```

You can get bootnode id from log of your first node.

# 🚀 Mainnet Ecosystem

To become part of the mainnet ecosystem, you can choose between two options:

1. **Archive Node**
2. **Validator Node**

### 🔰 <a name="archive-node">Archive Node</a>

An Archive Node functioning as a repository of historical blockchain data. By opting for an Archive Node, participants contribute to the network's resilience and accessibility, providing a valuable resource for the community. This role demands significant storage capacity and computational resources.

**Prerequisites (Minimal Setup):**

| Component | Configuration         |
| --------- | --------------------- |
| CPU       | 2 vCPUs (1 Core)      |
| RAM       | 4GB                   |
| Storage   | 250GB - SSD Preferred |
| OS        | Ubuntu 22.04 LTS      |

**Run Node**

1.  Create a user called `node` with optimal permissions
2.  Copy binary file to `/home/node/bin/jur-node`
3.  Copy [spec file](https://github.com/jurteam/jur-node/blob/fix/readme/res/jurMainnetSpecRaw.json) to `/home/node/jurMainnetSpecRaw.json`
4.  Create a `systemd` service file for the archive node. Typically, these files have a .service extension and are stored in the /etc/systemd/system/ directory.

```bash
sudo nano /etc/systemd/system/jur-node.service
```

5. Copy and Paste the following configuration into the file, adjusting the `<PUBLIC_IP>` and `<NAME>` accordingly:

```ini
# /etc/systemd/system/jur-node.service

[Unit]
Description=Jur Solochain Archive Node
After=network.target

[Service]
Type=simple
User=node
WorkingDirectory=/home/node
ExecStart=/home/node/jur-node --name="<NAME>" --db=rocksdb --pruning=archive \
        --telemetry-url "wss:/telemetry.polkadot.io/submit/ 1" \
        --rpc-port=9933 --rpc-external --rpc-cors=all --rpc-max-connections=300 \
        --public-addr=/ip4/<PUBLIC_IP>/tcp/30333 \
        --listen-addr=/ip4/0.0.0.0/tcp/30333 \
        --wasm-execution Compiled --prometheus-external --db-cache 512 \
        --base-path /home/node/jur-1-data \
        --chain /home/node/jurMainnetSpecRaw.json \
        --bootnodes "/ip4/172.27.152.16/tcp/30333/p2p/12D3KooWFYWEbk7AFck5wzEcdqeRfBkdsbWNuvcutHkkoUzNQqY3" \
        --bootnodes "/ip4/178.32.114.102/tcp/30333/ws/p2p/12D3KooWF8vpQQf1fwvCCLGpKE4rto8po1FpUXuVmZXenmRLzf7x" \
        --bootnodes "/ip4/178.32.114.155/tcp/30333/ws/p2p/12D3KooW9sMSrA6vsi1ZR4V6499TVvWdxWLHBTHJNpMhpSXV9eUn" \
        --reserved-nodes "/ip4/172.27.152.16/tcp/30333/p2p/12D3KooWFYWEbk7AFck5wzEcdqeRfBkdsbWNuvcutHkkoUzNQqY3" \
        --reserved-nodes "/ip4/178.32.114.102/tcp/30333/ws/p2p/12D3KooWF8vpQQf1fwvCCLGpKE4rto8po1FpUXuVmZXenmRLzf7x" \
        --reserved-nodes "/ip4/178.32.114.155/tcp/30333/ws/p2p/12D3KooW9sMSrA6vsi1ZR4V6499TVvWdxWLHBTHJNpMhpSXV9eUn"

Restart=on-failure
RestartSec=120
LimitNOFILE=16384

[Install]
WantedBy=multi-user.target
```

6. Reload the systemd manager to read the new service configuration:

```
sudo systemctl daemon-reload
```

7. Enable the service to start on boot:

```
sudo systemctl enable jur-node.service
```

8. Start the service:

```
sudo systemctl start jur-node.service
```

9. Check the status of the service to ensure it's running without errors:

```
sudo systemctl status jur-node.service
```

10. To see the detailed logs of the running service:

```
journalctl -f -u jur-node.service
```

<img  src="https://github.com/jurteam/jur-node/assets/4173518/304d1e26-259e-47a2-b3c7-7e251e1c7053">

📝 Note: Please be aware that Jur's mainnet ecosystem is currently exclusively admitting identified nodes through the use of the `--reserved-nodes` parameter. To have your node included in the peer list, it is necessary for any existing bootnode to grant permission for your node's IP and ID. Feel free to reach out to the community to request approval for your node.

### 🔰 <a name="validator-node">Validator Node</a>

A Validator Node actively participating in the consensus mechanism to validate and finalize transactions. Validator Nodes contribute to the security and integrity of the network.

**Prerequisites (Minimal Setup):**

| Component | Configuration        |
| --------- | -------------------- |
| CPU       | 2 vCPUs (1 Core)     |
| RAM       | 2GB                  |
| Storage   | 40GB - SSD Preferred |
| OS        | Ubuntu 22.04 LTS     |

**Run Node**

1. Create a user called `node` with optimal permissions
2. Copy binary file to `/home/node/bin/jur-node`
3. Copy [spec file](https://github.com/jurteam/jur-node/blob/fix/readme/res/jurMainnetSpecRaw.json) to `/home/node/jurMainnetSpecRaw.json`
4. Follow the instructions to create a systemd config file from the [Archive Node section](#archive-node). You need to remove `--pruning=archive` from the config and replace it with:

```ini
--pruning 1000  --validator
```

<img src="https://github.com/jurteam/jur-node/assets/4173518/b26d1065-b965-43f8-9239-c0185e6d2a2c">

📝 Note: For a validator node, it is essential to configure Aura and Grandpa keys following the guidelines outlined in the [Testnet Ecosystem](#testnet-ecosystem). Initially, we added 4 validator keys using POA (Proof of Authority) with Aura consensus to select validators from the keys stored on the chain.

Please be aware that Substrate currently does not support direct editing of the spec file after the initial addition of keys.

To address this challenge, several approaches can be considered:

- **Custom Pallet:** Create a custom pallet to amend the keys storage, allowing for the addition and removal of keys.
- **Sessions Pallet:** Utilize the sessions pallet to change the consensus mechanism. Migrate keys from Aura to the sessions pallet, which will then handle the addition and removal of keys.
- **Change Consensus to POS Stake:** Switch the entire consensus to Proof of Stake (POS). This transition will automatically be managed by the respective pallets.
- **Data Migration and Runtime Upgrade:** Explore the option of amending the keys database of the Aura and Grandpa pallets through data migration and runtime upgrade. Note that this solution requires thorough testing before implementation.

Presently, the community is considering on incorporating this feature. Once the feature attains a state of readiness, the community will proceed to activate it. Your patience and understanding are highly valued as we endeavor to enhance this functionality.

# 📦 Resources

- [Jur Telemetry](https://telemetry.polkadot.io/#list/0x58d1393b47b11707978fbc07e77d7b6f7d9aa88d207dc008a52385f7dba6156a)
- [Jur Explorer](https://jur.io/explorer/jur)
- [Mainnet ](https://polkadot.js.org/apps/?rpc=wss%3A%2F%2Fmainnet.jur.io#/explorer)
- [Testnet](https://polkadot.js.org/apps/?rpc=wss%3A%2F%2Ftestnet.jur.io#/explorer)
- [Mainnet Query Service](https://query.jur.io/)
- [Testnet Query Service](https://query-staging.jur.io/)
