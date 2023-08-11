# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).


## [Unreleased]


## [0.7.1] - 2023-08-11

### Fixed
- Release `state-fold-types` as well.


## [0.7.0] - 2023-08-10
### Changes
- Publish crates to crates.io instead of the `cartesi` private registry.
- Remove all mentions of the `cartesi` private registry.
- Add `eth-` prefix to all crate names. Since the crates are published
to the public registry, names had to be improved. This prefix in
particular was chosen as it is specific to the Ethereum base layer.
- Rewrite readme.
- Remove outdated examples.
- Trigger CI workflows only on relevant file types.
- Update dependencies.

### Fixed
- Apply all clippy suggestions; suppress warnings too complicated to
fix with low impact.


## [0.6.3] - 2023-03-09
### Changes
- Remove `structopt` and add `clap` for CLI configuration.
- Change `ethers` lib to 1.0.


## [0.6.2] - 2023-02-16
### Fix
- Version lock all crates.


## [0.6.1] - 2023-02-10
### Changes
- Add `env` option to `config`s.


## [0.6.0] - 2022-09-27
### Changes
- Mark gRPC client as `Send`.
- Update `prost` and `tonic` versions.

### Fix
- Fix test in newest `geth`.


## [0.5.0] - 2022-08-04
### Changes
- Make `block-subscriber` use a different provider for subscription,
reestablishing it at every subscription attempt.

- Add new field to `block-history` configuration for http endpoint.

### Fix
- Fix reorg detection bug not updating correctly.

- Make state-server-lib shutting down in case of subscription error.

- Make `block-history` configuration accept environment variables.


## [0.4.0] - 2022-08-01
### Changes
- Add `rustls` support

## [0.3.0] - 2022-07-26
### Changes
- Improve `state-fold` memory consumption through structural sharing.

- Add `block_archive` to `state-fold` for caching.

- Add configuration module to `state-server-lib`, and `state-client-lib`.

- Improve configuration modules in `block-archive` and `state-fold`.

- Add `config_utils` to `state-fold-types`.

- Add logging to `state-server-lib`.

- Improve the interface of `utils` module in `state-server-lib`.

- Upgrade `ethers` to `0.15`

  This version is at the time latest and includes fix for the `DiamondCut` ABIs being incorrectly generated which is not included in any published version yet

  See https://github.com/gakonst/ethers-rs/pull/1079/commits/2d734b2ec9cfb065c43bf4df91917fcfcc92390f

### Fixes
- Make variant `BlockArchiveError::EthersProviderError` in `block_archive` Send and Sync. This was preventing this error to be used properly in asynchronous code.

- Fix `simplestorage` test contract build failing with the new `ethers` by adding `ethers::` type of imports to the build script util for generating ABIs that replaces `ethers` imports to `state_fold_types::ethers`.

  Previously, it had supported only `ethers_core`, `ethers_contract` and `ethers_providers`.


## [0.2.2] - 2022-05-25
### Changed
- Make `contract::write` expect ABI on the input directly instead of getting it from the JSON under `abi` key.
- Make `release` workflow robust.

## [0.2.1] - 2022-05-18
### Added
- Add `release` workflow that publishes crates on `cartesi` registry.

### Changed
- Move `rustfmt` into its own workflow and run it on every push.
- Make `build` workflow run only on pull_request.
- Move `grpc-interfaces` to `state-server-common/grpc-interfaces` as it would not be included in the cargo package otherwise.

### Fixed
- Fix `clippy` issues automatically.

## [0.2.0] - 2022-05-16
### Added
- Add `cartesi` registry.
- Add CI configuration.
- Add user data field to `Foldable`.
- Store new `Foldable` trait in global cache.

### Changed
- Update outdated dependencies to latests release.
- Remove `offchain-utils` dependency.
- Remove `state-server-grpc`.
- Redesign state server, through crates `state-server-lib`, `state-client-lib`, and `state-server-common`.
- Remove outdated examples.
- Redesign `state-fold` interface, unifying both `StateFold` and `StateFoldDelegate` into a single `Foldable` trait.


## [0.1.1] 2022-04-28
### Changed
- Bumped grcp-interfaces to 0.7.0

## [0.1.0] - 2021-12-28
- Initial release

[Unreleased]: https://github.com/cartesi/state-fold/compare/v0.7.1...HEAD
[0.7.1]: https://github.com/cartesi/state-fold/compare/v0.7.0...v0.7.1
[0.7.0]: https://github.com/cartesi/state-fold/compare/v0.6.3...v0.7.0
[0.6.3]: https://github.com/cartesi/state-fold/compare/v0.6.2...v0.6.3
[0.6.2]: https://github.com/cartesi/state-fold/compare/v0.6.1...v0.6.2
[0.6.1]: https://github.com/cartesi/state-fold/compare/v0.6.0...v0.6.1
[0.6.0]: https://github.com/cartesi/state-fold/compare/v0.5.0...v0.6.0
[0.5.0]: https://github.com/cartesi/state-fold/compare/v0.4.0...v0.5.0
[0.4.0]: https://github.com/cartesi/state-fold/compare/v0.3.0...v0.4.0
[0.3.0]: https://github.com/cartesi/state-fold/compare/v0.2.2...v0.3.0
[0.2.2]: https://github.com/cartesi/state-fold/compare/v0.2.1...v0.2.2
[0.2.1]: https://github.com/cartesi/state-fold/compare/v0.2.0...v0.2.1
[0.2.0]: https://github.com/cartesi/state-fold/compare/v0.1.1...v0.2.0
[0.1.1]: https://github.com/cartesi/state-fold/compare/v0.1.0...v0.1.1
[0.1.0]: https://github.com/cartesi/state-fold/releases/tag/v0.1.0
