# Contributing to `typos`

Thanks for wanting to contribute! There are many ways to contribute and we
appreciate any level you're willing to do.

## Feature Requests

Need some new functionality to help?  You can let us know by opening an
[issue][new issue]. It's helpful to look through [all issues][all issues] in
case its already being talked about.

## Bug Reports

Please let us know about what problems you run into, whether in behavior or
ergonomics of API.  You can do this by opening an [issue][new issue]. It's
helpful to look through [all issues][all issues] in case its already being
talked about.

## Pull Requests

Looking for an idea? Check our [issues][issues]. If it's look more open ended,
it is probably best to post on the issue how you are thinking of resolving the
issue so you can get feedback early in the process. We want you to be
successful and it can be discouraging to find out a lot of re-work is needed.

Already have an idea?  It might be good to first [create an issue][new issue]
to propose it so we can make sure we are aligned and lower the risk of having
to re-work some of it and the discouragement that goes along with that.

### Updating the Dictionary

`typos` dictionary is a mapping of typos to a list of possible corrections (see [Design](docs/design.md)).  To add to the dictionary:

1. Add your typo to our data file `crates/typos-dict/assets/words.csv`

Format: `typo,correction[,correction...]`

2. Verify your change

Run
```bash
cargo run --package typos-dict-verify -- --input crates/typos-dict/assets/words.csv --output crates/typos-dict/assets/words.csv
```
Auto-cleans up your change according to some rules we have like:
- Don't prefer specific dialects in the dictionary, leaving those to [`varcon`](http://wordlist.aspell.net/varcon-readme/).
- Mixing up corrections and typos
- etc

3. Code-gen the dictionary

Run
```bash
cargo run --package typos-dict-codegen -- --output crates/typos-dict/src/dict_codegen.rs
```
(we do development-time code-gen to speed up builds)

### Process

When you first post a PR, we request that the commit history get cleaned
up.  We recommend avoiding this during the PR to make it easier to review how
feedback was handled. Once the commit is ready, we'll ask you to clean up the
commit history.  Once you let us know this is done, we can move forward with
merging!  If you are uncomfortable with these parts of git, let us know and we
can help.

We ask that all new files have the copyright header.  Please update the
copyright year for files you are modifying.

As a heads up, we'll be running your PR through the following gauntlet:
- warnings turned to compile errors
- `cargo test`
- `rustfmt`
- `clippy`
- `rustdoc`
- [`committed`](https://github.com/crate-ci/committed) to enforce [conventional commits](conventionalcommits.org/)

Check out our [CI][travis] for more information.

## Releasing

When we're ready to release, a project owner should do the following
- Determine what the next version is, according to semver
- Bump version in a commit
  - Update CHANGELOG.md
  - Update the version in `Cargo.toml`
  - Update the dependency version in `src/lib.rs`
  - Update the dependency version in `README.md`
- Tag the commit via `git tag -am "v<X>.<Y>.<Z>" v<X>.<Y>.<Z>`
- `git push upstream master --tag v<X>.<Y>.<Z>`
- Run `cargo publish` (run `cargo login` first if needed)

[issues]: https://github.com/crate-ci/typos/issues
[new issue]: https://github.com/crate-ci/typos/issues/new
[all issues]: https://github.com/crate-ci/typos/issues?utf8=%E2%9C%93&q=is%3Aissue
[travis]: https://github.com/crate-ci/typos/blob/master/.travis.yml
