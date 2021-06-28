# Change Log
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/)
and this project adheres to [Semantic Versioning](http://semver.org/).

<!-- next-header -->
## [Unreleased] - ReleaseDate

## [1.0.10] - 2021-06-28

#### Bug Fixes

- Remove reliance on compilation for pre-commit

## [1.0.9] - 2021-06-15

#### Bug Fixes

- Fix a crash from hitting a race condition

## [1.0.8] - 2021-06-15

## [1.0.7] - 2021-06-15

#### Features

- precommit hook settings

## [1.0.6] - 2021-06-07

#### Bug Fixes

- Fix the prior `typos <file>` fix that broke all other forms
- Extend the fix to other modes (`--dump-config`, etc)

## [1.0.5] - 2021-06-05

#### Bug Fixes

- Don't error out on `typos <file>`
- Reduce memory use when compiling for typos-vars

## [1.0.4] - 2021-05-31

#### Features

- Github Action support

## [1.0.3] - 2021-05-28

#### Bug Fixes

- Fix crash when processing stdin (`-`)

## [1.0.2] - 2021-05-28

#### Bug Fixes

- Don't panic when rendering typos on lines with non-ASCII character

## [1.0.1] - 2021-05-27

#### Bug Fixes

- Line numbers were off by `1 + <number of prior typos>`

## [1.0.0] - 2021-05-25

## [0.4.0] - 2021-05-21

#### Bug Fixes

- Correctly find config in parent directory
- Show abbreviated paths
- Check for word variations when also correcting a word
- Correct `ther` as not just `there` but also `the` and `their` (based on misspelling in Linux)
- Don't correct `hardlinked`
- `refernce` should correct to `reference` and not `references`

#### Performance

- Bypass variations, when possible

#### Features

- Log config loading to help debugging
- `typos`-specific ignores

## [0.3.0] - 2021-05-13

#### Bug Fixes

- Parsing identifiers according to the Unicode XID standard
- Corrected number detection

#### Performance

- Hand-rolled parser rather than regex
- Sped up UTF-8 validation
- Limited inner-loop asserts to debug builds
- Allow bypassing unicode cost with a  `--no-unicode` flag

#### Features

- Colored output support

## [0.2.0] - 2021-04-14

#### Bug Fixes

- Improve accuracy of typo column number
- Moved some reports to stderr
- Gracefully handle broken pipe
- Clearly defined exit codes

#### Features

- Fix support with `--write-changes`
- Diff support with `--diff`
- Locale-independent and locale-specific dictionaries
- Dictionary overrides
- UTF-16 file support
- Support for stdin corrections (with `-`)
- `--dump-config <path>` (with `-`) support
- Per-file type settings with custom file type support

#### Performance

- Multi-threading support
- Faster binary file detection
- Avoid looking up unknown words or numbers
- Small string optimizations
- Re-use config across arguments where possible

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
[Unreleased]: https://github.com/assert-rs/predicates-rs/compare/v1.0.10...HEAD
[1.0.10]: https://github.com/assert-rs/predicates-rs/compare/v1.0.9...v1.0.10
[1.0.9]: https://github.com/assert-rs/predicates-rs/compare/v1.0.8...v1.0.9
[1.0.8]: https://github.com/assert-rs/predicates-rs/compare/v1.0.7...v1.0.8
[1.0.7]: https://github.com/assert-rs/predicates-rs/compare/v1.0.6...v1.0.7
[1.0.6]: https://github.com/assert-rs/predicates-rs/compare/v1.0.5...v1.0.6
[1.0.5]: https://github.com/assert-rs/predicates-rs/compare/v1.0.4...v1.0.5
[1.0.4]: https://github.com/assert-rs/predicates-rs/compare/v1.0.3...v1.0.4
[1.0.3]: https://github.com/assert-rs/predicates-rs/compare/v1.0.2...v1.0.3
[1.0.2]: https://github.com/assert-rs/predicates-rs/compare/v1.0.1...v1.0.2
[1.0.1]: https://github.com/assert-rs/predicates-rs/compare/v1.0.0...v1.0.1
[1.0.0]: https://github.com/assert-rs/predicates-rs/compare/v0.4.0...v1.0.0
[0.4.0]: https://github.com/assert-rs/predicates-rs/compare/v0.3.0...v0.4.0
[0.3.0]: https://github.com/assert-rs/predicates-rs/compare/v0.2.0...v0.3.0
[0.2.0]: https://github.com/assert-rs/predicates-rs/compare/v0.1.4...v0.2.0
