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
