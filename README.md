<p align="center">
    <img src="https://github-production-user-asset-6210df.s3.amazonaws.com/4173518/297033697-a84bf93d-4c7b-40d7-a5e2-e107f39f55a0.png" alt="JurCahin Logo" />
</p>
<h1 align="center">Jur Node</h1>

Jur is the robust foundation that fuels Startup Societies, Countries, and Network States across the entire globe. We are breaking the mould of tired, outdated governance systems and ushering in the new: fair, just, collaborative, and beneficial to all.

Jur’s distinctive and powerful Layer-1 blockchain is built with Substrate, and will join the Polkadot Relay Chain as a parachain through Cumulus.

## 🏆 Requirements

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

By running the above command, all the necessary components will be pulled and the Jur node will be started in development mode.

### List of all commands

The following command can be used to explore all parameters and subcommands:

```
./target/release/jur-node -h
```

## 💻 Development Ecosystem (Single Node)

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

## 💡 Jur Testnet Ecosystem (Multi-Node)

The multi-node testnet environment provides a playground for developers to experiment with Jur chain features in a real scenario.

In order to set up the testnet, it is important to understand some key concepts. The following table summarizes the most common concepts and their descriptions:

| Node Type | What it does |
| --- | --- |
| Validator node | Processes the validating proofs to finalize a block |
| Boot node | Provides a static address and peer-to-peer (libp2p) public key that is used to bootstrap a node onto the network’s distributed hash table and to find peer nodes. |
| RPC node | Exposes an RPC interface over HTTP or WebSocket ports, so that users can read the blockchain state and submit transactions. There are often multiple RPC nodes behind a load balancer. |
| Archive node | Maintains all blocks starting from the genesis block with complete state available for every block. |
| Full node | Synchronizes with the chain to store the most recent block state and block headers for older blocks. |
| Aura Key | In the Aura consensus mechanism, a set of validators take turns proposing new blocks in a deterministic order. Each validator has an associated Aura key, and the validator whose turn it is to propose a block uses their Aura key to sign the block. This ensures a predictable and secure block creation process, as validators take turns being the leader. |
| Grandpa Key | Grandpa is responsible for finalizing blocks and ensuring the overall security of the blockchain. The Grandpa mechanism enhances the security and reliability of the blockchain by ensuring that once a block is finalized, it cannot be reverted. |

Proof of Authority (PoA) is a consensus algorithm used in the Jur Node to validate and confirm transactions. In a PoA system, network participants, known as validators or nodes, are pre-approved and identified entities with recognized authority or reputation.

We need to generate Aura and Grandpa keys to set up or join a multi-node Jur chain. Here are the commands to generate both keys:

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

Note: We need to run atleast three nodes in order to start producing and finalizing blocks.

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
