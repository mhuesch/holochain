# Changelog

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/). This project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## Unreleased

- Batch peer store write so we use less transactions. [#1007](https://github.com/holochain/holochain/pull/1007/).

## 0.0.109

- Make validation run concurrently up to 50 DhtOps. This allows us to make progress on other ops when waiting for the network. [\#1005](https://github.com/holochain/holochain/pull/1005)
- FIX: Prevent the conductor from trying to join cells to the network that are already in the process of joining. [\#1006](https://github.com/holochain/holochain/pull/1006)

## 0.0.108

- Refactor conductor to use parking lot rw lock instead of tokio rw lock. (Faster and prevents deadlocks.). [\#979](https://github.com/holochain/holochain/pull/979).

### Changed

- The scheduler should work now

## 0.0.107

## 0.0.106

### Changed

- All Holochain `Timestamp`s (including those in Headers) are now at the precision of microseconds rather than nanoseconds. This saves 4 bytes per timestamp in memory and on disk.
- Various database field names changed. **Databases created in prior versions will be incompatible.**
- HDK `sys_time` now returns a `holochain_zome_types::Timestamp` instead of a `core::time::Duration`.
- Exposes `UninstallApp` in the conductor admin API.

## 0.0.105

## 0.0.104

- Updates lair to 0.0.4 which pins rcgen to 0.8.11 to work around [https://github.com/est31/rcgen/issues/63](https://github.com/est31/rcgen/issues/63)

## 0.0.103

### Fixed

- This release solves the issues with installing happ bundles or registering DNA via the admin API concurrently. [\#881](https://github.com/holochain/holochain/pull/881).

### Changed

- Header builder now uses chain top timestamp for new headers if in the future
- Timestamps in headers require strict inequality in sys validation

## 0.0.102

### Known Issues :exclamation:

- We’ve become aware of a bug that locks up the conductor when installing happ bundles or registering DNA via the admin API concurrently. Please perform these actions sequentially until we’ve resolved the bug.

### Fixed

- Concurrent zome calls could cause the `init()` zome callback to run multiple times concurrently, causing `HeadMoved` errors. This is fixed, so that `init()` can only ever run once.
  - If a zome call has been waiting for another zome call to finish running `init()` for longer than 30 seconds, it will timeout.

### Changed

- Apps now have a more complex status. Apps now can be either enabled/disabled as well as running/stopped, the combination of which is captured by three distinctly named states:
  - “Running” (enabled + running) -\> The app is running normally
  - “Paused” (enabled + stopped) -\> The app is currently stopped due to some minor problem in one of its cells such as failed network access, but will start running again as soon as it’s able. Some Cells may still be running normally.
  - “Disabled” (disabled + stopped) -\> The app is stopped and will remain so until explicitly enabled via `EnableApp` admin method. Apps can be disabled manually via `DisableApp`, or automatically due to an unrecoverable error in a Cell.
- Some admin methods are deprecated due to the app status changes:
  - `ActivateApp` is deprecated in favor of `EnableApp`
  - `DeactivateApp` is deprecated in favor of `DisableApp`
- Apps will be automatically Paused if not all of their cells are able to join the network during startup

### Added

- `InstallAppBundle` command added to admin conductor API. [\#665](https://github.com/holochain/holochain/pull/665)
- `DnaSource` in conductor\_api `RegisterDna` call now can take a `DnaBundle` [\#665](https://github.com/holochain/holochain/pull/665)
- New admin interface methods:
  - `EnableApp` (replaces `ActivateApp`)
  - `DisableApp` (replaces `DeactivateApp`)
  - `StartApp` (used to attempt to manually restart a Paused app)
- Using the 3 level PLRU instance cache from latest holochain wasmer `v0.0.72`

## 0.0.101

This version contains breaking changes to the conductor API as well as a major upgrade to the underlying Wasm runtime.

***:exclamation: Performance impact***

The version of wasmer that is used in this holochain release contains bugs in the scoping of wasmer modules vs. instances, such that it blocks the proper release of memory and slows down execution of concurrent Wasm instances. While we were able to at least mitigate these effects and are coordinating with wasmer to find a proper solution as soon as possible.

The severity of these issues increases with cell concurrency, i.e. using multiple cells with the same DNA. Application development with a single conductor and a few cells are expected to work well unless your machine has serious resource restrictions.

### Added

- `InstallAppBundle` command added to admin conductor API. [\#665](https://github.com/holochain/holochain/pull/665)
- `DnaSource` in conductor\_api `RegisterDna` call now can take a `DnaBundle` [\#665](https://github.com/holochain/holochain/pull/665)

### Removed

- BREAKING:  `InstallAppDnaPayload` in admin conductor API `InstallApp` command now only accepts a hash.  Both properties and path have been removed as per deprecation warning.  Use either `RegisterDna` or `InstallAppBundle` instead. [\#665](https://github.com/holochain/holochain/pull/665)
- BREAKING: `DnaSource(Path)` in conductor\_api `RegisterDna` call now must point to `DnaBundle` as created by `hc dna pack` not a `DnaFile` created by `dna_util` [\#665](https://github.com/holochain/holochain/pull/665)

### CHANGED

- Updated to a version of `holochain_wasmer` that includes a migration to wasmer v2+. [\#773](https://github.com/holochain/holochain/pull/773/files), [\#801](https://github.com/holochain/holochain/pull/80), [\#836](https://github.com/holochain/holochain/pull/836)
- Introduced a simple instance cache to mitigate and potentially outweigh the effects of the aforementioned wasmer conditions [\#848](https://github.com/holochain/holochain/pull/848)

## 0.0.100

This is the first version number for the version of Holochain with a refactored state model (you may see references to it as Holochain RSM).

## 0.0.52-alpha2

*Note: Versions 0.0.52-alpha2 and older are belong to previous iterations of the Holochain architecture and are not tracked here.*
