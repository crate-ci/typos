# Change Log
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/)
and this project adheres to [Semantic Versioning](http://semver.org/).

<!-- next-header -->
## [Unreleased] - ReleaseDate

### Features

- Updated the dictionary with the [January 2025](https://github.com/crate-ci/typos/issues/1200) changes

## [1.29.5] - 2025-01-30

### Internal

- Update a dependency

## [1.29.4] - 2025-01-03

## [1.29.3] - 2025-01-02

## [1.29.2] - 2025-01-02

## [1.29.1] - 2025-01-02

### Fixes

- Don't correct `deriver`

## [1.29.0] - 2024-12-31

### Features

- Updated the dictionary with the [December 2024](https://github.com/crate-ci/typos/issues/1156) changes

### Performance

- Sped up dictionary lookups

## [1.28.4] - 2024-12-16

### Features

- `--format sarif` support

## [1.28.3] - 2024-12-12

### Fixes

- Correct `imlementations`, `includs`, `qurorum`, `transatctions`, `trasnactions`, `validasted`, `vview`

## [1.28.2] - 2024-12-02

### Fixes

- Don't correct `parametrize` variants

## [1.28.1] - 2024-11-26

### Fixes

- Add back in `lock` file types accidentally removed in 1.28 (`go.sum`, `requirements.txt`)

## [1.28.0] - 2024-11-25

### Features

- Updated the dictionary with the [November 2024](https://github.com/crate-ci/typos/issues/1139) changes
- Add many new types and file extensions to the `--type-list`, including ada, alire, bat, candid, carp, cml, devicetree, dita, dockercompose, grpbuild, graphql, hare, lean, meson, prolog, raku, reasonml, rescript, solidity, svelte, usd, v, wgsl

## [1.27.3] - 2024-11-08

### Fixes

- Don't correct `alloced`
- Don't correct `requestor`, a more domain specific variant of `requester`

## [1.27.2] - 2024-11-06

### Fixes

- Correct `fand`

## [1.27.1] - 2024-11-06

### Fixes

- Correct `alingment` as `alignment`, rather than `alinement`

## [1.27.0] - 2024-11-01

### Features

- Updated the dictionary with the [October 2024](https://github.com/crate-ci/typos/issues/1106) changes

## [1.26.8] - 2024-10-24

## [1.26.7] - 2024-10-24

## [1.26.6] - 2024-10-24

## [1.26.5] - 2024-10-24

## [1.26.4] - 2024-10-24

## [1.26.3] - 2024-10-24

### Fixes

- Accept `additionals`

## [1.26.2] - 2024-10-24

### Fixes

- Accept `tesselate` variants

## [1.26.1] - 2024-10-23

### Fixes

- Respect `--force-exclude` for binary files

## [1.26.0] - 2024-10-07

### Compatibility

- *(pre-commit)* Requires 3.2+

### Fixes

- *(pre-commit)* Resolve deprecations in 4.0 about deprecated stage names

## [1.25.0] - 2024-10-01

### Fixes

- Updated the dictionary with the [September 2024](https://github.com/crate-ci/typos/issues/1107) changes

## [1.24.6] - 2024-09-16

### Fixes

- Respect negation (`!`) in `extend-exclude`

## [1.24.5] - 2024-09-04

### Features

- *(action)* Support windows

## [1.24.4] - 2024-09-03

### Fixes

- Offer a correction for `grather`

## [1.24.3] - 2024-08-30

### Fixes

- Updated the dictionary with the [August 2024](https://github.com/crate-ci/typos/issues/1069) changes

## [1.24.2] - 2024-08-30

### Performance

- Cap unbounded parsing to avoid worst case performance (hit with test data)

## [1.24.1] - 2024-08-23

### Fixes

- Remove unverified varcon (locale data) entries

## [1.24.0] - 2024-08-23

### Features

- Update varcon (locale data) to version 2020.12.07

## [1.23.7] - 2024-08-22

### Fixes

- *(config)* Respect `--locale` / `default.locale` again after it was broken in 1.16.24

## [1.23.6] - 2024-07-31

### Fixes

- Updated the dictionary with the [July 2024](https://github.com/crate-ci/typos/issues/1051) changes

## [1.23.5] - 2024-07-25

### Features

- *(config)* Store config in `Cargo.toml`

## [1.23.4] - 2024-07-25

### Fixes

- Don't correct `countr_one` in C++

## [1.23.3] - 2024-07-22

### Fixes

- Fix `Dockerfile`

## [1.23.2] - 2024-07-10

### Features

- Automatically ignore JWT tokens

## [1.23.1] - 2024-07-05

### Fixes

- Add missing [June 2024](https://github.com/crate-ci/typos/issues/1024) changes

## [1.23.0] - 2024-07-05

### Fixes

- Updated the dictionary with the [June 2024](https://github.com/crate-ci/typos/issues/1024) changes

## [1.22.9] - 2024-06-22

### Fixes

- Stop correcting `reoccurrence`

## [1.22.8] - 2024-06-21

### Features

- *(action)* Add Arm, Mac support

## [1.22.7] - 2024-06-12

### Fixes

- Remove Linux arm64 binary support

## [1.22.6] - 2024-06-12

## [1.22.5] - 2024-06-12

### Features

- Linux arm64 binaries

## [1.22.4] - 2024-06-10

### Fixes

- Correct adventerous as adventurous instead of adventures
- Correct manifestion as manifestation instead of manifesto
- Correct manifestior as manifestation instead of manifesto

## [1.22.3] - 2024-06-07

### Fixes

- Don't correct `arbitral`

## [1.22.2] - 2024-06-07

## [1.22.1] - 2024-06-05

- In golang, `flate` is a valid term from the stdlib

## [1.22.0] - 2024-06-03

### Fixes

- Updated the dictionary with the [May 2024](https://github.com/crate-ci/typos/issues/1007) changes

## [1.21.0] - 2024-04-30

### Fixes

- Updated the dictionary with the [April 2024](https://github.com/crate-ci/typos/issues/956) changes

## [1.20.10] - 2024-04-23

### Fixes

- Don't correct `doas`, the OpenBSD command

## [1.20.9] - 2024-04-16

### Fixes

- Don't correct the unit `dBA` (as an identifier to limit to that case)

## [1.20.8] - 2024-04-12

### Fixes

- Don't correct `kms`
- Don't correct `inout`

## [1.20.7] - 2024-04-09

### Fixes

- Treat `.pyi` files as Python

## [1.20.6] - 2024-04-09

### Fixes

- Don't correct `automations`

## [1.20.5] - 2024-04-09

### Fixes

- Don't correct `hd`

## [1.20.4] - 2024-04-04

### Fixes

- Don't correct `ans`

## [1.20.3] - 2024-04-02

### Fixes

- Don't correct `arange` in Python
- Don't correct `EOFError` in Python

## [1.20.2] - 2024-04-02

### Fixes

- Don't correct `ang` ('angle' abbreviation)
- Don't correct `dur` ('duration' abbreviation)
- Don't correct `lst` (used in place of 'list' when it's a built-in)
- Don't correct `GUID` acronym
- Don't correct `IIS` acronym
- Don't correct `IME` acronym
- Don't correct `IOT` acronym
- Don't correct `ro` acronym
- Don't correct `ws` abbreviation
- Don't correct `JST` timezone
- Also correct `donw` to `done`

## [1.20.1] - 2024-04-01

### Fixes

- Don't correct `og` (OpenGraph's namespace), a regression from 1.20.0

### Fixes

- Don't correct `eof`, a regression from 1.20.0

## [1.20.0] - 2024-04-01

### Fixes

- Updated the dictionary with the [March 2024](https://github.com/crate-ci/typos/issues/943) changes
- Pull in typos from another source
- Don't correct `spile`

## [1.19.0] - 2024-03-01

### Fixes

- Updated the dictionary with the [February 2024](https://github.com/crate-ci/typos/issues/921) changes

## [1.18.2] - 2024-02-08

### Features

- Add `--sort` flag

### Fixes

- Protect against crash on multi-byte UTF-8 characters when parsed as ASCII

## [1.18.1] - 2024-02-05

### Fixes

- Don't correct derivate to derivative

## [1.18.0] - 2024-02-01

### Fixes

- Updated the dictionary with the [January 2024](https://github.com/crate-ci/typos/issues/897) changes

## [1.17.2] - 2024-01-19

### Fixes

- Treat `*.ts` as typescript, not typoscript
- Remove file type definitions that aren't used

## [1.17.1] - 2024-01-12

### Features

- First attempt at aarch64 for Mac

## [1.17.0] - 2024-01-03

### Fixes

- Updated the dictionary with the [November/December 2023](https://github.com/crate-ci/typos/issues/865) changes

## [1.16.26] - 2023-12-27

### Fixes

- Apply `extend-ignore-re` to file names in addition to file content

## [1.16.25] - 2023-12-13

### Fixes

- Have correction in `extend-words` match the original word's case

## [1.16.24] - 2023-12-08

### Fixes

- Don't silently ignore config when there is an error in a field

## [1.16.23] - 2023-11-06

### Fixes

- Ensure `--force-exclude` handles simple patterns like `some-dir`

## [1.16.22] - 2023-11-01

### Fixes

- Updated the dictionary with the [October 2023](https://github.com/crate-ci/typos/issues/843) changes

## [1.16.21] - 2023-10-27

### Fixes

- Improve colored output
- Don't correct `SHTTP` to `HTTPS`
- Associate `.jl` with the `jl` language type, rather than `lisp`
- Add Julia-specific allowed words

## [1.16.20] - 2023-10-16

### Features

- *(config)* `extend-ignore-words-re` field for defining all classes of words

## [1.16.19] - 2023-10-12

### Features

- `--file-list <PATH>` for accepting files to check from a file or stdin (`-`)

## [1.16.18] - 2023-10-09

### Features

- Add `--force-exclude` to exclude is always respected, even if explicitly passed in

## [1.16.17] - 2023-10-02

### Fixes

- Clean up `--help` output

## [1.16.16] - 2023-10-02

### Fixes

- Don't correct to typos `usefull` and `becuase`
- Updated the dictionary with the [September 2023](https://github.com/crate-ci/typos/issues/823) changes

## [1.16.15] - 2023-09-27

### Fixes

- Don't correct `O_WRONLY` in all cases

## [1.16.14] - 2023-09-25

### Fixes

- Don't correct `O_WRONLY`

## [1.16.13] - 2023-09-23

## [1.16.12] - 2023-09-18

### Performance

- Drop check times by about 20% with codegen-units

## [1.16.11] - 2023-09-06

### Features

- Support configuration in `pyproject.toml` files

## [1.16.10] - 2023-09-01

### Compatibility

- Bump MSRV to 1.70.0

### Fixes

- Updated the dictionary with the [August 2023](https://github.com/crate-ci/typos/issues/784) changes

## [1.16.9] - 2023-08-30

### Fixes

- Start correcting `arguement` again

## [1.16.8] - 2023-08-21

### Fixes

- *(action)* With `fetch-depth:0`, don't ignore excludes and fail with spaces in file names

## [1.16.7] - 2023-08-21

### Fixes

- Provide more context for IO error messages

## [1.16.6] - 2023-08-18

### Fixes

- Provide more context for IO error messages

## [1.16.5] - 2023-08-14

### Fixes

- Categorize `pnpm-lock.yaml` as a `lock` file

## [1.16.4] - 2023-08-12

## [1.16.3] - 2023-08-09

### Fixes

- Set bad exit code on disallowed words
- Allow `Nd` in man pages
- Allow `nd` in CSS
- Allow `ot` and `stap` in sh

## [1.16.2] - 2023-08-01

### Fixes

- Updated the dictionary with the [July 2023](https://github.com/crate-ci/typos/issues/777) changes

## [1.16.1] - 2023-07-14

### Performance

- Small optimization to parser

## [1.16.0] - 2023-07-10

### Features

- If a file type doesn't exist for `*.in`, then try matching `*`

## [1.15.10] - 2023-07-03

### Fixes

- Updated the dictionary with the [June 2023](https://github.com/crate-ci/typos/issues/733) changes

## [1.15.9] - 2023-06-30

### Fixes

- Don't correct "contiguities"

## [1.15.8] - 2023-06-29

### Fixes

- Don't correct some valid words

## [1.15.7] - 2023-06-26

### Fixes

- Show corrections in priority order again (broken in 1.15.0)

## [1.15.6] - 2023-06-26

### Fixes

- Don't correct currency code "zar"

## [1.15.5] - 2023-06-22

### Fixes

- Don't correct `typos.toml`

## [1.15.4] - 2023-06-22

### Fixes

- Correctly merge default setting for a file type with a user's file type settings

## [1.15.3] - 2023-06-21

### Fixes

- User config overrides default glob definitions

## [1.15.2] - 2023-06-20

### Fixes

- Ensure atomic output is locked

## [1.15.1] - 2023-06-19

### Fixes

- Don't correct `accreting`

## [1.15.0] - 2023-06-08

### Fixes

- Major dictionary update

## [1.14.12] - 2023-06-02

### Fixes

- Updated the dictionary with the [May 2023](https://github.com/crate-ci/typos/issues/718) changes

## [1.14.11] - 2023-05-22

### Fixes

- *(action)* Don't require `sudo`

## [1.14.10] - 2023-05-19

### Fixes

- Don't correct `add-ons` to `add-owns`

## [1.14.9] - 2023-05-03

### Fixes

- Updated the dictionary with the [April 2023](https://github.com/crate-ci/typos/issues/705) changes

## [1.14.8] - 2023-04-19

### Performance

- *(pre-commit)* Build musl wheels

## [1.14.7] - 2023-04-19

### Fixes

- *(pre-commit)* Ensure there is a default target to install for `typos-src`

## [1.14.6] - 2023-04-13

### Internal

- Update dependency

## [1.14.5] - 2023-03-30

### Fixes

- Fix UTF-16 support broken in 1.14.4

## [1.14.4] - 2023-03-30

### Fixes

- Updated the dictionary with the [March 2023](https://github.com/crate-ci/typos/issues/677) changes

### Internal

- Changed the UTF-16 encoder / decoder

## [1.14.3] - 2023-03-18

### Internal

- Update dependency

## [1.14.2] - 2023-03-18

### Features

- *(config)* `extend-ignores-re` for extending support for ignoring non-identifiers like email, PGP signatures, etc

## [1.14.1] - 2023-03-18

### Fixes

- *(config)* Actually support `extend-ignore-identifiers-re`

## [1.14.0] - 2023-03-18

### Features

- *(config)* `extend-ignore-identifiers-re` field for defining all classes of identifiers

## [1.13.26] - 2023-03-16

### Internal

- Update dependency

## [1.13.25] - 2023-03-16

### Fixes

- Don't correct `commitish`

## [1.13.24] - 2023-03-14

### Fixes

- `CLICOLOR=1` now works correctly
- `NO_COLOR=` now works correctly
- Auto-enable colors in CI

## [1.13.23] - 2023-03-14

### Performance

- *(action)* Avoid docker builds

## [1.13.22] - 2023-03-13

### Fixes

- Correct `existend` to `existent` in addition to `existed`
- Correct `erronerous` to `erroneous`

## [1.13.21] - 2023-03-13

### Fixes

- Correct `empheral` to `ephemeral`
- Ignore `go.mod` by default
- Ensure pre-commit isn't slow to install

## [1.13.20] - 2023-03-07

### Features

Publish to PyPI (second attempt)

## [1.13.19] - 2023-03-07

### Features

Publish to PyPI (first attempt)

## [1.13.18] - 2023-03-06

### Compatibility

- Pre-built Linux binaries will now be built with Ubuntu 20.04, rather than 18.04

## [1.13.17] - 2023-03-06

### Fixes

- Misc corrections

## [1.13.16] - 2023-02-28

## [1.13.15] - 2023-02-28

### Internal

- Dependency updates

## [1.13.14] - 2023-02-27

### Fixes

- Correct encrypt / decrypt related words

## [1.13.13] - 2023-02-27

### Fixes

- Correct `grouepd` to `grouped`

## [1.13.12] - 2023-02-23

### Features

- *(action)* Allow writing changes

## [1.13.11] - 2023-02-23

### Fixes

- Don't correct `referer`

## [1.13.10] - 2023-02-01

### Fixes

- Correct `detctable` & `seaonal` & `wayferer`

## [1.13.9] - 2023-01-25

### Fixes

- Correct `regylar` -> `regular`
- Do not correct substitutents
- Do not correct substituters

## [1.13.8] - 2023-01-16

### Fixes

- Correct serialzie -> serialize

## [1.13.7] - 2023-01-14

### Features

- `--file-types` debug flag to debug file type detection issues

## [1.13.6] - 2022-12-20

### Features

- *(precommit)* Add a docker variant

## [1.13.5] - 2022-12-19

- *(docker)* Ensure correct libc is available
- *(precommit)* Restrict what stages it runs during

## [1.13.4] - 2022-12-06

### Fixes

- Don't correct `nilable` as it's used by the Ruby community

## [1.13.3] - 2022-12-02

## [1.13.2] - 2022-12-02

## [1.13.1] - 2022-11-30

### Fixes

- Don't crash in non-UTF8 cases with `--format brief`
- Report correct column with `--format brief`

## [1.13.0] - 2022-11-22

### Fixes

- Over a hundred new corrections

## [1.12.14] - 2022-11-04

## [1.12.13] - 2022-11-04

### Fixes

- Don't crash on `--locale en-us`

## [1.12.12] - 2022-10-25

### Fixes

- Correct decreypted -> decrypted

## [1.12.11] - 2022-10-20

### Fixes

- Correct `wrappning`

## [1.12.10] - 2022-10-11

### Fixes

- Several more corrections

## [1.12.9] - 2022-10-06

### Fixes

- Correct `whaat` to `what`

## [1.12.8] - 2022-09-28

### Fixes

- Polished help output

## [1.12.7] - 2022-09-22

### Fixes

- Correct `targest` to `target`

## [1.12.6] - 2022-09-22

### Fixes

- Correct `pararmeter` to `parameter`

## [1.12.5] - 2022-09-15

### Fixes

- Correct `stte` to `state`

## [1.12.4] - 2022-09-08

### Fixes

- Don't correct `NDArray` in Python

## [1.12.3] - 2022-09-06

### Fixes

- Add more typos

## [1.12.2] - 2022-09-01

### Fixes

- Ignore `thead` always, HTML is too pervasive

## [1.12.1] - 2022-09-01

### Fixes

- Ignore `thead` tag also in markdown

## [1.12.0] - 2022-08-30

### Fixes

- Many new corrections

## [1.11.5] - 2022-08-29

### Fixes

- Ignore `thead` tag also in tsx/jsx

## [1.11.4] - 2022-08-25

### Fixes

- Ignore CSS hex numbers starting with decimal values, like #111AAA

## [1.11.3] - 2022-08-25

### Fixes

- Ignore `thead` for CSS

## [1.11.2] - 2022-08-23

### Fixes

- Correct "inappropriate[ly]"
- Ignore `thead` tag only in HTML
- Ignore `windo` in vim
- Narrow scope of ignoring `flate` to the `flate2` identifier

## [1.11.1] - 2022-08-16

### Fixes

- Don't correct `thead` tag
- Correct `deffer` to either `differ` or `defer`
- Correct `opauqe` to `opaque`

## [1.11.0] - 2022-08-13

### Fixes

- Added many more corrections

## [1.10.3] - 2022-07-22

### Fixes

- Correct `anonimised`, `anonimized`

## [1.10.2] - 2022-06-22

### Fixes

- *(Github Action)* Don't add annotation for binary files

## [1.10.1] - 2022-06-16

### Fixes

- When stdout is redirected to a file, don't spell check that file

## [1.10.0] - 2022-06-16

### Features

- *(Github Action)* Report typos as annotations

### Performance

- *(Github Action)* Only check the files changed in a PR

## [1.9.0] - 2022-06-15

### Fixes

- Made overlapping file definitions deterministic (most specific one wins) (#500)

## [1.8.1] - 2022-05-16

- Extra debug logging

## [1.8.0] - 2022-05-10

### Fixes

- Actually ignore items, like hashes, at the end of input
- Actually ignore items, like hashes, that have trailing backslashes
- Better detect short base64's by watching the padding bytes

## [1.7.3] - 2022-04-28

### Fixes

- Fix alignment in reports for numbers, broken in 1.7.2
- Correct `identitiy`

## [1.7.2] - 2022-04-28

### Fixes

- Fix misalignment in the detailed report with multiwidth characters
- Fix report to show columns as 1-indexed

## [1.7.1] - 2022-04-25

### Fixes

- Ignore uppercase UUID because Microsoft
- Correct `unencyrpted`
- Correct `signign`

## [1.7.0] - 2022-04-18

### Fixes

- Ignore CSS Colors

## [1.6.0] - 2022-04-06

### Fixes

- Treat `go.mod` as go-lang source like we do manifests for other languages
- Treat `go.sum` as a lock file, ignoring it by default

## [1.5.0] - 2022-03-09

### Compatibility

- File-types in the default config were moved to being built-in
- Lock files have moved to the same file type, regardless of syntax

### Fixes

- Don't spell check lock files as the user shouldn't have to manage their config to handle transitive dependency names

## [1.4.1] - 2022-02-14

#### Fixes

- Improve URL detection to avoid spell checking them

## [1.4.0] - 2022-02-08

#### Fixes

- Many new typos added

## [1.3.9] - 2022-01-26

#### Fixes

- Attempt to detect base64 values shorter than 90 characters

## [1.3.8] - 2022-01-26

#### Fixes

- Don't stop parsing on `%`, `\\` when outside of an escape sequence or printf interpolation

## [1.3.7] - 2022-01-24

#### Fixes

- Don't complain when mixing ordinals with markdown

## [1.3.6] - 2022-01-24

#### Fixes

- Don't error on `type` settings in config

## [1.3.5] - 2022-01-21

## [1.3.4] - 2022-01-12

## [1.3.3] - 2021-12-18

#### Fixes

- Hopefully fix pre-commit hook on cygwin

## [1.3.2] - 2021-12-14

#### Fixes

- Correct "requierment" to "requirement", not "requirements"
- Correct "descrepancy" to "discrepancy"

## [1.3.1] - 2021-11-16

## [1.3.0] - 2021-11-15

Note: MSRV is now 1.54

#### Fixes

- Fix multiple escape sequences in a row
- Large batch of additional corrections
- Use static CRT for pre-built Windows binaries

## [1.2.1] - 2021-11-03

## [1.2.0] - 2021-10-23

#### Bug Fixes

- Remove some overhead
- Smarter color control
- Remove some general false positives
- Remove some Rust-specific false positives
- Check language packaging with language (due to overlap of dependency names
- Skip checking lock files since they are machine generated
- Fix default/override config overlaying

## [1.1.9] - 2021-09-14

## [1.1.8] - 2021-08-30

#### Bug Fixes

- Correct `surrouned` to `surround` and now `surrounded`

## [1.1.7] - 2021-08-20

#### Bug Fixes

- Improve hex/hash detection

## [1.1.6] - 2021-08-06

#### Bug Fixes

- Add `instantialed` typo

## [1.1.5] - 2021-08-04

#### Bug Fixes

- Reduce false-positives by not checking file contents of certs

## [1.1.4] - 2021-08-02

#### Bug Fixes

- Don't stop parsing at c-escape but continue on

## [1.1.3] - 2021-07-30

#### Bug Fixes

- Reduce false-positives by ignoring words following possible c-escape sequences or printf patterns.

## [1.1.2] - 2021-07-30

#### Bug Fixes

- `wasn,was` correction causes problems with `wasn't`

## [1.1.1] - 2021-07-27

#### Bug Fixes

- Correct the Linux binary link after switching to musl

## [1.1.0] - 2021-07-27

#### Features

- Add more corrections

## [1.0.11] - 2021-06-29

#### Change of Behavior

- `ignore-hex` and `identifier-leading-digit` are deprecated and `typos` acts as
  if `ignore-hex=true` and `identifier-leading-digit=false`.

#### Features

- Automatically ignore
  - UUIDs
  - SHAs
  - base64 encoded data (must be at least 90 bytes)
  - emails
  - URLs

#### Performance

- Due to new literal detection, finding identifiers is takes 10x longer.
  Combined with word splitting, it only takes 3x longer.  The majority of the
  time is spent in dictionary lookups, so we don't expect this to have too much impact in the end.

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
[Unreleased]: https://github.com/crate-ci/typos/compare/v1.29.5...HEAD
[1.29.5]: https://github.com/crate-ci/typos/compare/v1.29.4...v1.29.5
[1.29.4]: https://github.com/crate-ci/typos/compare/v1.29.3...v1.29.4
[1.29.3]: https://github.com/crate-ci/typos/compare/v1.29.2...v1.29.3
[1.29.2]: https://github.com/crate-ci/typos/compare/v1.29.1...v1.29.2
[1.29.1]: https://github.com/crate-ci/typos/compare/v1.29.0...v1.29.1
[1.29.0]: https://github.com/crate-ci/typos/compare/v1.28.4...v1.29.0
[1.28.4]: https://github.com/crate-ci/typos/compare/v1.28.3...v1.28.4
[1.28.3]: https://github.com/crate-ci/typos/compare/v1.28.2...v1.28.3
[1.28.2]: https://github.com/crate-ci/typos/compare/v1.28.1...v1.28.2
[1.28.1]: https://github.com/crate-ci/typos/compare/v1.28.0...v1.28.1
[1.28.0]: https://github.com/crate-ci/typos/compare/v1.27.3...v1.28.0
[1.27.3]: https://github.com/crate-ci/typos/compare/v1.27.2...v1.27.3
[1.27.2]: https://github.com/crate-ci/typos/compare/v1.27.1...v1.27.2
[1.27.1]: https://github.com/crate-ci/typos/compare/v1.27.0...v1.27.1
[1.27.0]: https://github.com/crate-ci/typos/compare/v1.26.8...v1.27.0
[1.26.8]: https://github.com/crate-ci/typos/compare/v1.26.7...v1.26.8
[1.26.7]: https://github.com/crate-ci/typos/compare/v1.26.6...v1.26.7
[1.26.6]: https://github.com/crate-ci/typos/compare/v1.26.5...v1.26.6
[1.26.5]: https://github.com/crate-ci/typos/compare/v1.26.4...v1.26.5
[1.26.4]: https://github.com/crate-ci/typos/compare/v1.26.3...v1.26.4
[1.26.3]: https://github.com/crate-ci/typos/compare/v1.26.2...v1.26.3
[1.26.2]: https://github.com/crate-ci/typos/compare/v1.26.1...v1.26.2
[1.26.1]: https://github.com/crate-ci/typos/compare/v1.26.0...v1.26.1
[1.26.0]: https://github.com/crate-ci/typos/compare/v1.25.0...v1.26.0
[1.25.0]: https://github.com/crate-ci/typos/compare/v1.24.6...v1.25.0
[1.24.6]: https://github.com/crate-ci/typos/compare/v1.24.5...v1.24.6
[1.24.5]: https://github.com/crate-ci/typos/compare/v1.24.4...v1.24.5
[1.24.4]: https://github.com/crate-ci/typos/compare/v1.24.3...v1.24.4
[1.24.3]: https://github.com/crate-ci/typos/compare/v1.24.2...v1.24.3
[1.24.2]: https://github.com/crate-ci/typos/compare/v1.24.1...v1.24.2
[1.24.1]: https://github.com/crate-ci/typos/compare/v1.24.0...v1.24.1
[1.24.0]: https://github.com/crate-ci/typos/compare/v1.23.7...v1.24.0
[1.23.7]: https://github.com/crate-ci/typos/compare/v1.23.6...v1.23.7
[1.23.6]: https://github.com/crate-ci/typos/compare/v1.23.5...v1.23.6
[1.23.5]: https://github.com/crate-ci/typos/compare/v1.23.4...v1.23.5
[1.23.4]: https://github.com/crate-ci/typos/compare/v1.23.3...v1.23.4
[1.23.3]: https://github.com/crate-ci/typos/compare/v1.23.2...v1.23.3
[1.23.2]: https://github.com/crate-ci/typos/compare/v1.23.1...v1.23.2
[1.23.1]: https://github.com/crate-ci/typos/compare/v1.23.0...v1.23.1
[1.23.0]: https://github.com/crate-ci/typos/compare/v1.22.9...v1.23.0
[1.22.9]: https://github.com/crate-ci/typos/compare/v1.22.8...v1.22.9
[1.22.8]: https://github.com/crate-ci/typos/compare/v1.22.7...v1.22.8
[1.22.7]: https://github.com/crate-ci/typos/compare/v1.22.6...v1.22.7
[1.22.6]: https://github.com/crate-ci/typos/compare/v1.22.5...v1.22.6
[1.22.5]: https://github.com/crate-ci/typos/compare/v1.22.4...v1.22.5
[1.22.4]: https://github.com/crate-ci/typos/compare/v1.22.3...v1.22.4
[1.22.3]: https://github.com/crate-ci/typos/compare/v1.22.2...v1.22.3
[1.22.2]: https://github.com/crate-ci/typos/compare/v1.22.1...v1.22.2
[1.22.1]: https://github.com/crate-ci/typos/compare/v1.22.0...v1.22.1
[1.22.0]: https://github.com/crate-ci/typos/compare/v1.21.0...v1.22.0
[1.21.0]: https://github.com/crate-ci/typos/compare/v1.20.10...v1.21.0
[1.20.10]: https://github.com/crate-ci/typos/compare/v1.20.9...v1.20.10
[1.20.9]: https://github.com/crate-ci/typos/compare/v1.20.8...v1.20.9
[1.20.8]: https://github.com/crate-ci/typos/compare/v1.20.7...v1.20.8
[1.20.7]: https://github.com/crate-ci/typos/compare/v1.20.6...v1.20.7
[1.20.6]: https://github.com/crate-ci/typos/compare/v1.20.5...v1.20.6
[1.20.5]: https://github.com/crate-ci/typos/compare/v1.20.4...v1.20.5
[1.20.4]: https://github.com/crate-ci/typos/compare/v1.20.3...v1.20.4
[1.20.3]: https://github.com/crate-ci/typos/compare/v1.20.2...v1.20.3
[1.20.2]: https://github.com/crate-ci/typos/compare/v1.20.1...v1.20.2
[1.20.1]: https://github.com/crate-ci/typos/compare/v1.20.0...v1.20.1
[1.20.0]: https://github.com/crate-ci/typos/compare/v1.19.0...v1.20.0
[1.19.0]: https://github.com/crate-ci/typos/compare/v1.18.2...v1.19.0
[1.18.2]: https://github.com/crate-ci/typos/compare/v1.18.1...v1.18.2
[1.18.1]: https://github.com/crate-ci/typos/compare/v1.18.0...v1.18.1
[1.18.0]: https://github.com/crate-ci/typos/compare/v1.17.2...v1.18.0
[1.17.2]: https://github.com/crate-ci/typos/compare/v1.17.1...v1.17.2
[1.17.1]: https://github.com/crate-ci/typos/compare/v1.17.0...v1.17.1
[1.17.0]: https://github.com/crate-ci/typos/compare/v1.16.26...v1.17.0
[1.16.26]: https://github.com/crate-ci/typos/compare/v1.16.25...v1.16.26
[1.16.25]: https://github.com/crate-ci/typos/compare/v1.16.24...v1.16.25
[1.16.24]: https://github.com/crate-ci/typos/compare/v1.16.23...v1.16.24
[1.16.23]: https://github.com/crate-ci/typos/compare/v1.16.22...v1.16.23
[1.16.22]: https://github.com/crate-ci/typos/compare/v1.16.21...v1.16.22
[1.16.21]: https://github.com/crate-ci/typos/compare/v1.16.20...v1.16.21
[1.16.20]: https://github.com/crate-ci/typos/compare/v1.16.19...v1.16.20
[1.16.19]: https://github.com/crate-ci/typos/compare/v1.16.18...v1.16.19
[1.16.18]: https://github.com/crate-ci/typos/compare/v1.16.17...v1.16.18
[1.16.17]: https://github.com/crate-ci/typos/compare/v1.16.16...v1.16.17
[1.16.16]: https://github.com/crate-ci/typos/compare/v1.16.15...v1.16.16
[1.16.15]: https://github.com/crate-ci/typos/compare/v1.16.14...v1.16.15
[1.16.14]: https://github.com/crate-ci/typos/compare/v1.16.13...v1.16.14
[1.16.13]: https://github.com/crate-ci/typos/compare/v1.16.12...v1.16.13
[1.16.12]: https://github.com/crate-ci/typos/compare/v1.16.11...v1.16.12
[1.16.11]: https://github.com/crate-ci/typos/compare/v1.16.10...v1.16.11
[1.16.10]: https://github.com/crate-ci/typos/compare/v1.16.9...v1.16.10
[1.16.9]: https://github.com/crate-ci/typos/compare/v1.16.8...v1.16.9
[1.16.8]: https://github.com/crate-ci/typos/compare/v1.16.7...v1.16.8
[1.16.7]: https://github.com/crate-ci/typos/compare/v1.16.6...v1.16.7
[1.16.6]: https://github.com/crate-ci/typos/compare/v1.16.5...v1.16.6
[1.16.5]: https://github.com/crate-ci/typos/compare/v1.16.4...v1.16.5
[1.16.4]: https://github.com/crate-ci/typos/compare/v1.16.3...v1.16.4
[1.16.3]: https://github.com/crate-ci/typos/compare/v1.16.2...v1.16.3
[1.16.2]: https://github.com/crate-ci/typos/compare/v1.16.1...v1.16.2
[1.16.1]: https://github.com/crate-ci/typos/compare/v1.16.0...v1.16.1
[1.16.0]: https://github.com/crate-ci/typos/compare/v1.15.10...v1.16.0
[1.15.10]: https://github.com/crate-ci/typos/compare/v1.15.9...v1.15.10
[1.15.9]: https://github.com/crate-ci/typos/compare/v1.15.8...v1.15.9
[1.15.8]: https://github.com/crate-ci/typos/compare/v1.15.7...v1.15.8
[1.15.7]: https://github.com/crate-ci/typos/compare/v1.15.6...v1.15.7
[1.15.6]: https://github.com/crate-ci/typos/compare/v1.15.5...v1.15.6
[1.15.5]: https://github.com/crate-ci/typos/compare/v1.15.4...v1.15.5
[1.15.4]: https://github.com/crate-ci/typos/compare/v1.15.3...v1.15.4
[1.15.3]: https://github.com/crate-ci/typos/compare/v1.15.2...v1.15.3
[1.15.2]: https://github.com/crate-ci/typos/compare/v1.15.1...v1.15.2
[1.15.1]: https://github.com/crate-ci/typos/compare/v1.15.0...v1.15.1
[1.15.0]: https://github.com/crate-ci/typos/compare/v1.14.12...v1.15.0
[1.14.12]: https://github.com/crate-ci/typos/compare/v1.14.11...v1.14.12
[1.14.11]: https://github.com/crate-ci/typos/compare/v1.14.10...v1.14.11
[1.14.10]: https://github.com/crate-ci/typos/compare/v1.14.9...v1.14.10
[1.14.9]: https://github.com/crate-ci/typos/compare/v1.14.8...v1.14.9
[1.14.8]: https://github.com/crate-ci/typos/compare/v1.14.7...v1.14.8
[1.14.7]: https://github.com/crate-ci/typos/compare/v1.14.6...v1.14.7
[1.14.6]: https://github.com/crate-ci/typos/compare/v1.14.5...v1.14.6
[1.14.5]: https://github.com/crate-ci/typos/compare/v1.14.4...v1.14.5
[1.14.4]: https://github.com/crate-ci/typos/compare/v1.14.3...v1.14.4
[1.14.3]: https://github.com/crate-ci/typos/compare/v1.14.2...v1.14.3
[1.14.2]: https://github.com/crate-ci/typos/compare/v1.14.1...v1.14.2
[1.14.1]: https://github.com/crate-ci/typos/compare/v1.14.0...v1.14.1
[1.14.0]: https://github.com/crate-ci/typos/compare/v1.13.26...v1.14.0
[1.13.26]: https://github.com/crate-ci/typos/compare/v1.13.25...v1.13.26
[1.13.25]: https://github.com/crate-ci/typos/compare/v1.13.24...v1.13.25
[1.13.24]: https://github.com/crate-ci/typos/compare/v1.13.23...v1.13.24
[1.13.23]: https://github.com/crate-ci/typos/compare/v1.13.22...v1.13.23
[1.13.22]: https://github.com/crate-ci/typos/compare/v1.13.21...v1.13.22
[1.13.21]: https://github.com/crate-ci/typos/compare/v1.13.20...v1.13.21
[1.13.20]: https://github.com/crate-ci/typos/compare/v1.13.19...v1.13.20
[1.13.19]: https://github.com/crate-ci/typos/compare/v1.13.18...v1.13.19
[1.13.18]: https://github.com/crate-ci/typos/compare/v1.13.17...v1.13.18
[1.13.17]: https://github.com/crate-ci/typos/compare/v1.13.16...v1.13.17
[1.13.16]: https://github.com/crate-ci/typos/compare/v1.13.15...v1.13.16
[1.13.15]: https://github.com/crate-ci/typos/compare/v1.13.14...v1.13.15
[1.13.14]: https://github.com/crate-ci/typos/compare/v1.13.13...v1.13.14
[1.13.13]: https://github.com/crate-ci/typos/compare/v1.13.12...v1.13.13
[1.13.12]: https://github.com/crate-ci/typos/compare/v1.13.11...v1.13.12
[1.13.11]: https://github.com/crate-ci/typos/compare/v1.13.10...v1.13.11
[1.13.10]: https://github.com/crate-ci/typos/compare/v1.13.9...v1.13.10
[1.13.9]: https://github.com/crate-ci/typos/compare/v1.13.8...v1.13.9
[1.13.8]: https://github.com/crate-ci/typos/compare/v1.13.7...v1.13.8
[1.13.7]: https://github.com/crate-ci/typos/compare/v1.13.6...v1.13.7
[1.13.6]: https://github.com/crate-ci/typos/compare/v1.13.5...v1.13.6
[1.13.5]: https://github.com/crate-ci/typos/compare/v1.13.4...v1.13.5
[1.13.4]: https://github.com/crate-ci/typos/compare/v1.13.3...v1.13.4
[1.13.3]: https://github.com/crate-ci/typos/compare/v1.13.2...v1.13.3
[1.13.2]: https://github.com/crate-ci/typos/compare/v1.13.1...v1.13.2
[1.13.1]: https://github.com/crate-ci/typos/compare/v1.13.0...v1.13.1
[1.13.0]: https://github.com/crate-ci/typos/compare/v1.12.14...v1.13.0
[1.12.14]: https://github.com/crate-ci/typos/compare/v1.12.13...v1.12.14
[1.12.13]: https://github.com/crate-ci/typos/compare/v1.12.12...v1.12.13
[1.12.12]: https://github.com/crate-ci/typos/compare/v1.12.11...v1.12.12
[1.12.11]: https://github.com/crate-ci/typos/compare/v1.12.10...v1.12.11
[1.12.10]: https://github.com/crate-ci/typos/compare/v1.12.9...v1.12.10
[1.12.9]: https://github.com/crate-ci/typos/compare/v1.12.8...v1.12.9
[1.12.8]: https://github.com/crate-ci/typos/compare/v1.12.7...v1.12.8
[1.12.7]: https://github.com/crate-ci/typos/compare/v1.12.6...v1.12.7
[1.12.6]: https://github.com/crate-ci/typos/compare/v1.12.5...v1.12.6
[1.12.5]: https://github.com/crate-ci/typos/compare/v1.12.4...v1.12.5
[1.12.4]: https://github.com/crate-ci/typos/compare/v1.12.3...v1.12.4
[1.12.3]: https://github.com/crate-ci/typos/compare/v1.12.2...v1.12.3
[1.12.2]: https://github.com/crate-ci/typos/compare/v1.12.1...v1.12.2
[1.12.1]: https://github.com/crate-ci/typos/compare/v1.12.0...v1.12.1
[1.12.0]: https://github.com/crate-ci/typos/compare/v1.11.5...v1.12.0
[1.11.5]: https://github.com/crate-ci/typos/compare/v1.11.4...v1.11.5
[1.11.4]: https://github.com/crate-ci/typos/compare/v1.11.3...v1.11.4
[1.11.3]: https://github.com/crate-ci/typos/compare/v1.11.2...v1.11.3
[1.11.2]: https://github.com/crate-ci/typos/compare/v1.11.1...v1.11.2
[1.11.1]: https://github.com/crate-ci/typos/compare/v1.11.0...v1.11.1
[1.11.0]: https://github.com/crate-ci/typos/compare/v1.10.3...v1.11.0
[1.10.3]: https://github.com/crate-ci/typos/compare/v1.10.2...v1.10.3
[1.10.2]: https://github.com/crate-ci/typos/compare/v1.10.1...v1.10.2
[1.10.1]: https://github.com/crate-ci/typos/compare/v1.10.0...v1.10.1
[1.10.0]: https://github.com/crate-ci/typos/compare/v1.9.0...v1.10.0
[1.9.0]: https://github.com/crate-ci/typos/compare/v1.8.1...v1.9.0
[1.8.1]: https://github.com/crate-ci/typos/compare/v1.8.0...v1.8.1
[1.8.0]: https://github.com/crate-ci/typos/compare/v1.7.3...v1.8.0
[1.7.3]: https://github.com/crate-ci/typos/compare/v1.7.2...v1.7.3
[1.7.2]: https://github.com/crate-ci/typos/compare/v1.7.1...v1.7.2
[1.7.1]: https://github.com/crate-ci/typos/compare/v1.7.0...v1.7.1
[1.7.0]: https://github.com/crate-ci/typos/compare/v1.6.0...v1.7.0
[1.6.0]: https://github.com/crate-ci/typos/compare/v1.5.0...v1.6.0
[1.5.0]: https://github.com/crate-ci/typos/compare/v1.4.1...v1.5.0
[1.4.1]: https://github.com/crate-ci/typos/compare/v1.4.0...v1.4.1
[1.4.0]: https://github.com/crate-ci/typos/compare/v1.3.9...v1.4.0
[1.3.9]: https://github.com/crate-ci/typos/compare/v1.3.8...v1.3.9
[1.3.8]: https://github.com/crate-ci/typos/compare/v1.3.7...v1.3.8
[1.3.7]: https://github.com/crate-ci/typos/compare/v1.3.6...v1.3.7
[1.3.6]: https://github.com/crate-ci/typos/compare/v1.3.5...v1.3.6
[1.3.5]: https://github.com/crate-ci/typos/compare/v1.3.4...v1.3.5
[1.3.4]: https://github.com/crate-ci/typos/compare/v1.3.3...v1.3.4
[1.3.3]: https://github.com/crate-ci/typos/compare/v1.3.2...v1.3.3
[1.3.2]: https://github.com/crate-ci/typos/compare/v1.3.1...v1.3.2
[1.3.1]: https://github.com/crate-ci/typos/compare/v1.3.0...v1.3.1
[1.3.0]: https://github.com/crate-ci/typos/compare/v1.2.1...v1.3.0
[1.2.1]: https://github.com/crate-ci/typos/compare/v1.2.0...v1.2.1
[1.2.0]: https://github.com/crate-ci/typos/compare/v1.1.9...v1.2.0
[1.1.9]: https://github.com/crate-ci/typos/compare/v1.1.8...v1.1.9
[1.1.8]: https://github.com/crate-ci/typos/compare/v1.1.7...v1.1.8
[1.1.7]: https://github.com/crate-ci/typos/compare/v1.1.6...v1.1.7
[1.1.6]: https://github.com/crate-ci/typos/compare/v1.1.5...v1.1.6
[1.1.5]: https://github.com/crate-ci/typos/compare/v1.1.4...v1.1.5
[1.1.4]: https://github.com/crate-ci/typos/compare/v1.1.3...v1.1.4
[1.1.3]: https://github.com/crate-ci/typos/compare/v1.1.2...v1.1.3
[1.1.2]: https://github.com/crate-ci/typos/compare/v1.1.1...v1.1.2
[1.1.1]: https://github.com/crate-ci/typos/compare/v1.1.0...v1.1.1
[1.1.0]: https://github.com/crate-ci/typos/compare/v1.0.11...v1.1.0
[1.0.11]: https://github.com/crate-ci/typos/compare/v1.0.10...v1.0.11
[1.0.10]: https://github.com/crate-ci/typos/compare/v1.0.9...v1.0.10
[1.0.9]: https://github.com/crate-ci/typos/compare/v1.0.8...v1.0.9
[1.0.8]: https://github.com/crate-ci/typos/compare/v1.0.7...v1.0.8
[1.0.7]: https://github.com/crate-ci/typos/compare/v1.0.6...v1.0.7
[1.0.6]: https://github.com/crate-ci/typos/compare/v1.0.5...v1.0.6
[1.0.5]: https://github.com/crate-ci/typos/compare/v1.0.4...v1.0.5
[1.0.4]: https://github.com/crate-ci/typos/compare/v1.0.3...v1.0.4
[1.0.3]: https://github.com/crate-ci/typos/compare/v1.0.2...v1.0.3
[1.0.2]: https://github.com/crate-ci/typos/compare/v1.0.1...v1.0.2
[1.0.1]: https://github.com/crate-ci/typos/compare/v1.0.0...v1.0.1
[1.0.0]: https://github.com/crate-ci/typos/compare/v0.4.0...v1.0.0
[0.4.0]: https://github.com/crate-ci/typos/compare/v0.3.0...v0.4.0
[0.3.0]: https://github.com/crate-ci/typos/compare/v0.2.0...v0.3.0
[0.2.0]: https://github.com/crate-ci/typos/compare/v0.1.4...v0.2.0
