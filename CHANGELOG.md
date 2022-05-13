# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

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

[Unreleased]: https://github.com/cartesi-corp/state-fold/compare/v0.2.1...HEAD
[0.2.1]: https://github.com/cartesi-corp/state-fold/compare/v0.2.0...v0.2.1
[0.2.0]: https://github.com/cartesi-corp/state-fold/compare/v0.1.1...v0.2.0
[0.1.1]: https://github.com/cartesi-corp/state-fold/compare/v0.1.0...v0.1.1
[0.1.0]: https://github.com/cartesi-corp/state-fold/releases/tag/v0.1.0
