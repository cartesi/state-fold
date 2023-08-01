# ![State Fold Picture](https://cartesi.notion.site/image/https%3A%2F%2Fs3-us-west-2.amazonaws.com%2Fsecure.notion-static.com%2F0fd9b7e1-93a3-4c99-8680-eb28cbc6aa3e%2F2023_LinkedIn_Header_-_Core_Contributors_(1).png?table=block&id=9eea3776-7397-4739-81ad-c96172f72c1e&spaceId=62ffa304-a896-4e7e-823d-af99eb3cccf3&width=2000&userId=&cache=v2 "State Fold")

# State Fold

A collection of libraries with GRPC server and client for reading custom-defined state at any position on the Ethereum blockchain accessed via any Ethereum JSON-RPC API provider.

## Description

Whether you want to reach for historical **blocks** / **states** or subscribe to get the latest **blocks** / **states** as they are added to the network, both of these use-cases were considered when designing *State Fold*.

*State Fold Server Library* has methods to **read state** and analogous ones to **read blocks**. You can read in bulk and way back in the history, but also **subscribe** to receive new **states**, or analogously **blocks**, as they get added on the blockchain.

The structural definition of **state** is up to the developer that is using this library. The type definition propagates all the way up to the server and client libraries, i.e. to the API schema. Design of this approach uses DRY attitude. That means we just want you to write the type once and make everything generic so that you get the server for no extra effort.

Developer also defines the behavior of **syncing** and **folding** to read **state**, where `sync` means calculating state based on hard-coded **initial state** and `fold` means calculating state based on **previous state**.

## Requirements

* [Rust 1.50+ stable](https://rustup.rs/)

### For examples

* [Solc 0.7.5+](https://docs.soliditylang.org/en/v0.8.11/installing-solidity.html#linux-packages)
* [Geth 1.9.24+](https://geth.ethereum.org/docs/install-and-build/installing-geth)

## Installation

This repository is a `cargo` workspace. Crates are published on `cartesi` crates registry. That's a private registry, so you need to add this into your `~/.cargo/config.toml`:

```toml
[registries]
cartesi = { index = "https://github.com/cartesi/crates-index" }
```

After that you can add one or more *State Fold* crates into your `Cargo.toml` like so:

```toml
[dependencies]
# Common crate for all other crates
state-fold-types = { version = "0.6", registry = "cartesi" }

# Libraries for state-fold core logic
block-history = { version = "0.6", registry = "cartesi" }
state-fold = { version = "0.6", registry = "cartesi" }

# Common crate for server/client
state-server-common = { version = "0.6", registry = "cartesi", features = ["server"] }

# Libraries for state-fold GRPC interfaces
state-server-lib = { version = "0.6", registry = "cartesi" }
state-client-lib = { version = "0.6", registry = "cartesi" }
```

## Usage

### Local development environment

Build all targets of all crates:

```
cargo build --all-targets --all-features
```

Run all tests:

```
cargo test
```

Format code:

```
cargo fmt --all
```

## Examples

See [`examples/` directory](examples).

## Documentation

* [Cartesi Rollups Rollout — State Fold @ Mediun](https://medium.com/cartesi/state-fold-cfe5f4d79639)

## License

The library is licensed under [Apache-2.0](LICENSE).
