# Jur-Node

The Network State for the Digital Economy built on top of the Polkadot ecosystem.

## Getting Started

Follow the steps below to get started with the Jur-Node:

### Rust Setup

First, complete the [basic Rust setup instructions](./docs/rust-setup.md).

### Run

Use Rust's native `cargo` command to build and launch the jur-node:

```sh
cargo run --release -- --dev
```

### Build

The `cargo run` command will perform an initial build. Use the following command to build the node
without launching it:

```sh
cargo build --release
```

### Embedded Docs

Once the project has been built, the following command can be used to explore all parameters and
subcommands:

```sh
./target/release/jur-node -h
```

## Run

The provided `cargo run` command will launch a temporary node and its state will be discarded after
you terminate the process. After the project has been built, there are other ways to launch the
node.

### Single-Node Development Chain

This command will start the single-node development chain with non-persistent state:

```bash
./target/release/jur-node --dev
```

Purge the development chain's state:

```bash
./target/release/jur-node purge-chain --dev
```

Start the development chain with detailed logging:

```bash
RUST_BACKTRACE=1 ./target/release/jur-node -ldebug --dev
```

> Development chain means that the state of our chain will be in a tmp folder while the nodes are
> running. Also, **alice** account will be authority and sudo account as declared in the
> [genesis state](https://github.com/jurteam/jur-node/blob/8fe1a147a4a7437cf991fa5ee0921679d189af41/node/src/chain_spec.rs#L51).
> At the same time the following accounts will be pre-funded:
> - Alice
> - Bob
> - Alice//stash
> - Bob//stash

In case of being interested in maintaining the chain' state between runs a base path must be added
so the db can be stored in the provided folder instead of a temporal one. We could use this folder
to store different chain databases, as a different folder will be created per different chain that
is ran. The following commands shows how to use a newly created folder as our db base path.

```bash
// Create a folder to use as the db base path
$ mkdir my-chain-state

// Use of that folder to store the chain state
$ ./target/release/jur-node --dev --base-path ./my-chain-state/

// Check the folder structure created inside the base path after running the chain
$ ls ./my-chain-state
chains
$ ls ./my-chain-state/chains/
dev
$ ls ./my-chain-state/chains/dev
db keystore network
```


### Connect with Polkadot-JS Apps Front-end

Once the node template is running locally, you can connect it with **Polkadot-JS Apps** front-end
to interact with your chain. [Click
here](https://polkadot.js.org/apps/#/explorer?rpc=ws://localhost:9944) connecting the Apps to your
jur node.

### Multi-Node Jur Testnet

## Generate Keys
Generate Aura and Grandpa session keys using this [tutorial](https://docs.substrate.io/tutorials/get-started/trusted-network/#generate-your-account-and-keys)

If validators are not part of chain_spec, follow `Add validators` section of this [tutorial](https://docs.substrate.io/tutorials/get-started/trusted-network/#create-a-custom-chain-specification)

## Add keys to the keystore

* For Node 1

```
./target/release/jur-node key insert --base-path  /tmp/node01 \
--chain jur-testnet \
--scheme Sr25519 \
--suri <first_keyset-aura-secret-seed> \
--key-type aura


./target/release/jur-node key insert --base-path  /tmp/node01 \
--chain jur-testnet \
--scheme Ed25519 \
--suri <first_keyset-granpa-secret-seed> \
--key-type gran
```

* For Node 2

```
./target/release/jur-node key insert --base-path  /tmp/node02 \
--chain jur-testnet \
--scheme Sr25519 \
--suri <second_keyset-aura-secret-seed> \
--key-type aura


./target/release/jur-node key insert --base-path  /tmp/node02 \
--chain jur-testnet \
--scheme Ed25519 \
--suri <second_keyset-granpa-secret-seed> \
--key-type gran
```

* For Node 3

```
./target/release/jur-node key insert --base-path  /tmp/node03 \
--chain jur-testnet \
--scheme Sr25519 \
--suri <third_keyset-aura-secret-seed> \
--key-type aura


./target/release/jur-node key insert --base-path  /tmp/node03 \
--chain jur-testnet \
--scheme Ed25519 \
--suri <third_keyset-granpa-secret-seed> \
--key-type gran
```

## Start First Node

```
./target/release/jur-node \
  --base-path /tmp/node01 \
  --chain jur-testnet \
  --port 30333 \
  --ws-port 9945 \
  --rpc-port 9933 \
  --validator \
  --rpc-methods Unsafe \
  --name MyNode01
```


## Start Second Node

```
./target/release/jur-node \
  --base-path /tmp/node02 \
  --chain jur-testnet \
  --port 30334 \
  --ws-port 9946 \
  --rpc-port 9934 \
  --validator \
  --rpc-methods Unsafe \
  --name MyNode02 \
  --bootnodes <Set Boot Nodes>
```

## Start Third Node

You can now allow other validators to join the network using the `--bootnodes` and `--validator` command-line options.
```
./target/release/jur-node \
  --base-path /tmp/node03 \
  --chain jur-testnet \
  --port 30335 \
  --ws-port 9947 \
  --rpc-port 9935 \
  --validator \
  --rpc-methods Unsafe \
  --name MyNode02 \
  --bootnodes <Set Boot Nodes>
```













