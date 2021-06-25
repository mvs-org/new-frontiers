# MetaverseVM Node

Enter the [Metaverse](https://mvs.org). THE NEW REALITY :rocket:

## Getting Started

Follow the steps below to take control and compile your own node. :hammer_and_wrench:

### Rust Setup

First, complete the [basic Rust setup instructions](./doc/rust-setup.md).

### Build

The `cargo run` command will perform an initial build. Use the following command to build the node
without launching it:

```sh
cargo build --release
```

### Run

Launch one metaverse node:

```sh
./target/release/metaversevm --validator --tmp --rpc-cors all 
```



Launch a test net with 3 metaverse node at localhost :

On console for node01:

```sh
./target/release/metaversevm --validator --tmp --rpc-cors all \
    --node-key 0000000000000000000000000000000000000000000000000000000000000111
```

On console for node02:

```
./target/release/metaversevm --validator --tmp --rpc-cors all \
    --port 30334 \
    --ws-port 9946 \
    --rpc-port 9934 \
    --bootnodes /ip4/127.0.0.1/tcp/30333/p2p/12D3KooWRBaMZHnFYtT1B143sSLHkx8G6ysPSg8PMzqXkymm38Ld
```

On console for node03:
```
./target/release/metaversevm --validator --tmp --rpc-cors all \
    --port 30335 \
    --ws-port 9947 \
    --rpc-port 9935 \
    --bootnodes /ip4/127.0.0.1/tcp/30333/p2p/12D3KooWRBaMZHnFYtT1B143sSLHkx8G6ysPSg8PMzqXkymm38Ld
```

### GPU mining

**Install miner**

Access the project's [releases page](https://github.com/ethereum-mining/ethminer/releases), and pick up the latest Linux tarball. Unpack the tarball in the directory where you want to run Ethminer. It's a pre-compiled binary, so that's all you need to do before you start using it.

```sh
$ mkdir ethminer
$ wget -O ethminer/ethminer.tar.gz https://github.com/ethereum-mining/ethminer/releases/download/v0.18.0/ethminer-0.18.0-cuda-9-linux-x86_64.tar.gz
$ tar xzf ethminer/ethminer.tar.gz -C ethminer/
$ ethminer/bin/ethminer --help
ethminer 0.18.0
Build: linux/release/gnu

Ethminer - GPU ethash miner
minimal usage : ethminer [DEVICES_TYPE] [OPTIONS] -P... [-P...]
```

**Run miner**

```sh
// Change to correct RPC port to get work
ethminer/bin/ethminer -P http://127.0.0.1:9933
```

### CPU mining(only for test)

**Dependencies:**

Linux-based:

```
sudo apt-get install libleveldb-dev libcurl4-openssl-dev libmicrohttpd-dev libudev-dev cmake
```

macOS:

```
brew install leveldb libmicrohttpd
```

**Install:**

```
git clone --depth=1 https://github.com/avatar-lavventura/ethminer.git 
cd ethminer
./scripts/install_deps.sh
```

**Build:**

```
cmake -H. -Bbuild
cd build/ethminer
make -j $(nproc)
```

**Run miner**

```sh
// Change to correct RPC port to get work
./ethminer -F http://localhost:9933 --mining-threads 4
```

**Notice**

If the compilation doesn't work through, it could be the C++ compiler version problem, please follow these step:

```sh
vi cmake/EthCompilerSettings.cmake
...
# comment out this line
#add_compile_options(-Werror)
...
...
# at last line add
add_compile_options($<$<COMPILE_LANGUAGE:CXX>:-Wno-deprecated-copy>)
add_compile_options($<$<COMPILE_LANGUAGE:CXX>:-Wno-implicit-fallthrough>)
add_compile_options($<$<COMPILE_LANGUAGE:CXX>:-Wno-maybe-uninitialized>)
...
```

### Embedded Docs

Once the project has been built, the following command can be used to explore all parameters and
subcommands:

```sh
./target/release/metaverse-vm -h
```

## Run

The provided `cargo run` command will launch a temporary node and its state will be discarded after
you terminate the process. After the project has been built, there are other ways to launch the
node.

### Single-Node Development Chain

This command will start the single-node development chain with persistent state:

```bash
./target/release/metaverse-vm --dev
```

Purge the development chain's state:

```bash
./target/release/metaverse-vm purge-chain --dev
```

Start the development chain with detailed logging:

```bash
RUST_LOG=debug RUST_BACKTRACE=1 ./target/release/metaverse-vm -lruntime=debug --dev
```

## Template Structure

Metaverse is built using the Substrate Blockchain Framework. A Substrate project such as this consists of a number of components that are spread across a few
directories.

### Node

A blockchain node is an application that allows users to participate in a blockchain network. Nodes expose a number of capabilities:

-   Networking: Substrate nodes use the [`libp2p`](https://libp2p.io/) networking stack to allow the
    nodes in the network to communicate with one another.
-   Consensus: Blockchains must have a way to come to
    [consensus](https://substrate.dev/docs/en/knowledgebase/advanced/consensus) on the state of the
    network. Substrate makes it possible to supply custom consensus engines and also ships with
    several consensus mechanisms that have been built on top of
    [Web3 Foundation research](https://research.web3.foundation/en/latest/polkadot/NPoS/index.html).
-   RPC Server: A remote procedure call (RPC) server is used to interact with Substrate nodes.

There are several files in the `node` directory - take special note of the following:

-   [`chain_spec.rs`](./node/src/chain_spec.rs): A
    [chain specification](https://substrate.dev/docs/en/knowledgebase/integrate/chain-spec) is a
    source code file that defines a Substrate chain's initial (genesis) state. Chain specifications
    are useful for development and testing, and critical when architecting the launch of a
    production chain. Take note of the `development_config` and `testnet_genesis` functions, which
    are used to define the genesis state for the local development chain configuration. These
    functions identify some
    [well-known accounts](https://substrate.dev/docs/en/knowledgebase/integrate/subkey#well-known-keys)
    and use them to configure the blockchain's initial state.
-   [`service.rs`](./node/src/service.rs): This file defines the node implementation. Take note of
    the libraries that this file imports and the names of the functions it invokes. In particular,
    there are references to consensus-related topics, such as the
    [longest chain rule](https://substrate.dev/docs/en/knowledgebase/advanced/consensus#longest-chain-rule),
    the [Aura](https://substrate.dev/docs/en/knowledgebase/advanced/consensus#aura) block authoring
    mechanism and the
    [GRANDPA](https://substrate.dev/docs/en/knowledgebase/advanced/consensus#grandpa) finality
    gadget.

After the node has been [built](#build), refer to the embedded documentation to learn more about the
capabilities and configuration parameters that it exposes:

```shell
./target/release/metaverse-vm --help
```


### Run in Docker

First, install [Docker](https://docs.docker.com/get-docker/) and
[Docker Compose](https://docs.docker.com/compose/install/).

Then run the following command to start a single node development chain.

```bash
./scripts/docker_run.sh
```

This command will firstly compile your code, and then start a local development network. You can
also replace the default command (`cargo build --release && ./target/release/metaverse-vm --dev --ws-external`)
by appending your own. A few useful ones are as follow.

```bash
# Run Substrate node without re-compiling
./scripts/docker_run.sh ./target/release/metaverse-vm --dev --ws-external

# Purge the local dev chain
./scripts/docker_run.sh ./target/release/metaverse-vm purge-chain --dev

# Check whether the code is compilable
./scripts/docker_run.sh cargo check
```
