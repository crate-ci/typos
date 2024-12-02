# Contributing to `typos`

Thanks for wanting to contribute! There are many ways to contribute and we
appreciate any level you're willing to do.

## Feature Requests

Need some new functionality to help?  You can let us know by opening an
[issue][new issue]. It's helpful to look through [all issues][all issues] in
case it's already being talked about.

## Bug Reports

Please let us know about what problems you run into, whether in behavior or
ergonomics of API.  You can do this by opening an [issue][new issue]. It's
helpful to look through [all issues][all issues] in case it's already being
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

`typos` dictionary is a mapping of typos to a list of possible corrections (see [Design](docs/design.md)).

If you aren't in a hurry, [we have a pinned
Issue](https://github.com/crate-ci/typos/issues) to collect dictionary changes
to be done in bulk in an attempt to lower the barrier for improving the dictionary.

Otherwise, to add to the dictionary:

1. Add your typo to our data file `crates/typos-dict/assets/words.csv`

Format: `typo,correction[,correction...]`

2. Code-gen the dictionary

With `cargo` and `rustfmt` installed, run
```console
$ SNAPSHOTS=overwrite cargo test --workspace
```
(we do development-time code-gen to speed up builds)

3. Verify your change

Run
```console
$ cargo test --workspace
```
Auto-cleans up your change according to some rules we have like:
- Don't prefer specific dialects in the dictionary, leaving those to [`varcon`](http://wordlist.aspell.net/varcon-readme/).
- Mixing up corrections and typos
- etc

### Process

As a heads up, we'll be running your PR through the following gauntlet:
- warnings turned to compile errors
- `cargo test`
- `rustfmt`
- `clippy`
- `rustdoc`
- [`committed`](https://github.com/crate-ci/committed) as we use [Conventional](https://www.conventionalcommits.org) commit style
- [`typos`](https://github.com/crate-ci/typos) to check spelling

Not everything can be checked automatically though.

We request that the commit history gets cleaned up.
We ask that commits are atomic, meaning they are complete and have a single responsibility.
PRs should tell a cohesive story, with test and refactor commits that keep the
fix or feature commits simple and clear.

Specifically, we would encourage
- File renames be isolated into their own commit
- Add tests in a commit before their feature or fix, showing the current behavior.
  The diff for the feature/fix commit will then show how the behavior changed,
  making it clearer to reviewers and the community and showing people that the
  test is verifying the expected state.
  - e.g. [clap#5520](https://github.com/clap-rs/clap/pull/5520)

Note that we are talking about ideals.
We understand having a clean history requires more advanced git skills;
feel free to ask us for help!
We might even suggest where it would work to be lax.
We also understand that editing some early commits may cause a lot of churn
with merge conflicts which can make it not worth editing all of the history.

For code organization, we recommend
- Grouping `impl` blocks next to their type (or trait)
- Grouping private items after the `pub` item that uses them.
  - The intent is to help people quickly find the "relevant" details, allowing them to "dig deeper" as needed.  Or put another way, the `pub` items serve as a table-of-contents.
  - The exact order is fuzzy; do what makes sense

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
[CI]: https://github.com/crate-ci/typos/tree/master/.github/workflows
