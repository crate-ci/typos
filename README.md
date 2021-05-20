# typos

> **Source code spell checker**

Finds and corrects spelling mistakes among source code:
- Fast enough to run on monorepos
- Low false positives so you can run on PRs

![Screenshot](./docs/screenshot.png)

[![Build Status](https://dev.azure.com/crate-ci/crate-ci/_apis/build/status/typos?branchName=master)](https://dev.azure.com/crate-ci/crate-ci/_build/latest?definitionId=11&branchName=master)
[![codecov](https://codecov.io/gh/crate-ci/typos/branch/master/graph/badge.svg)](https://codecov.io/gh/crate-ci/typos)
[![Documentation](https://img.shields.io/badge/docs-master-blue.svg)][Documentation]
![License](https://img.shields.io/crates/l/typos.svg)
[![Crates Status](https://img.shields.io/crates/v/typos.svg)](https://crates.io/crates/typos)

Dual-licensed under [MIT](LICENSE-MIT) or [Apache 2.0](LICENSE-APACHE)

## Documentation

- [Installation](#install)
- [Reference](docs/reference.md)
- [Comparison with other spell checkers](docs/comparison.md)
- [Benchmarks](benchsuite/runs)
- [Design](docs/design.md)
- [Contribute](CONTRIBUTING.md)
- [CHANGELOG](CHANGELOG.md)

## Install

[Download](https://github.com/crate-ci/typos/releases) a pre-built binary
(installable via [gh-install](https://github.com/crate-ci/gh-install).

Or use rust to install:
```bash
cargo install typos-cli
```

[Crates.io]: https://crates.io/crates/typos-cli
[Documentation]: https://docs.rs/typos
