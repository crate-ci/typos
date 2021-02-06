# Change Log
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/)
and this project adheres to [Semantic Versioning](http://semver.org/).

<!-- next-header -->
## [Unreleased] - ReleaseDate

## 0.1.4 - 2019-11-03


#### Bug Fixes

*   Ignore numbers as identifiers ([a00831c8](https://github.com/crate-ci/typos/commit/a00831c847b7efd81be520ea9b5d02f70555351f))
*   Improve the organization of --help ([a48a457c](https://github.com/crate-ci/typos/commit/a48a457cc3ca817850118e2a2fb8b20fecdd40b8))

#### Features

*   Dump files, identifiers, and words ([ce365ae1](https://github.com/crate-ci/typos/commit/ce365ae12e12fddfb6fc42a7f1e5ea71834d6051), closes [#41](https://github.com/crate-ci/typos/issues/41))
*   Give control over allowed identifier characters for leading vs rest ([107308a6](https://github.com/crate-ci/typos/commit/107308a655a425eb593bf5e4928572c16e6a9bdd))

#### Performance

*   Use standard identifier rules to avoid doing umber checks ([107308a6](https://github.com/crate-ci/typos/commit/107308a655a425eb593bf5e4928572c16e6a9bdd))
*   Only do hex check if digits are in identifiers ([68cd36d0](https://github.com/crate-ci/typos/commit/68cd36d0de90226dbc9d31c2ce6d8bf6b69adb5c))

<!-- next-url -->
[Unreleased]: https://github.com/assert-rs/predicates-rs/compare/v0.1.4...HEAD
