# pre-commit

To use `typos` with [`pre-commit`](https://pre-commit.com), point its
config at this repository:

```yaml
repos:
  - repo: https://github.com/crate-ci/typos
    rev: v1.29.0
    hooks:
      - id: typos
```

The `typos` id installs a prebuilt executable from GitHub releases. If
one does not exist for the target platform, or if one built from
sources is preferred, use `typos-docker` (requires Docker), or `typos-src`
(requires Rust) as the hook id instead.

Be sure to change `rev` to use the desired `typos` git tag or
revision.

The hook configuration defaults to writing fixes, which triggers a
`pre-commit` failure if any files are modified. To make it report its
findings only instead, override the hook's `args` with something that
does not contain `-w`/`--write-changes`, for example `[]` (meaning
pass no options).
