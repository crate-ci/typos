# GitHub Action

If you want an easy way to test your repository spelling (or a subset of files)
you can use the Typos Action!

```yaml
name: Spelling

permissions:
  contents: read

on: [pull_request]

env:
  CLICOLOR: 1

jobs:
  spelling:
    name: Spell Check with Typos
    runs-on: ubuntu-latest
    steps:
    - name: Checkout Actions Repository
      uses: actions/checkout@v4
    - name: Spell Check Repo
      uses: crate-ci/typos@v1.29.5
```

**Requirements:** The runner must have `wget` installed
(see
[#769](https://github.com/crate-ci/typos/issues/769)
[#1191](https://github.com/crate-ci/typos/issues/1191)
).

## Input

| Name               | Description                                                     | Required | Default                                              |
| ------------------ | --------------------------------------------------------------- | -------- | ---------------------------------------------------- |
| files              | Files or patterns to check                                      | false    | If not defined, the default set of files are checked |
| extend_identifiers | Comma separated list of extend identifiers, like someone's name | false    | not set                                              |
| extend_words       | Comma separated list of extend words.                           | false    | not set                                              |
| isolated           | Ignore implicit configuration files                             | false    | false                                                |
| write_changes      | Writes changes on the Action's local checkout                   | false    | false                                                |
| config             | Use a custom config file (must exist)                           | false    | not set                                              |

`write_changes`: doesn't commit or push anything to the branch. It only writes the changes locally
to disk, and this can be combined with other actions, for instance that will [submit code
suggestions based on that local diff](https://github.com/getsentry/action-git-diff-suggestions).

### Examples

```yaml
name: Test GitHub Action
on: [pull_request]

jobs:
  run:
    name: Spell Check with Typos
    runs-on: ubuntu-latest
    steps:
    - name: Checkout Actions Repository
      uses: actions/checkout@v4

    - name: Check spelling of file.txt
      uses: crate-ci/typos@v1.29.5
      with:
        files: ./file.txt

    - name: Use custom config file
      uses: crate-ci/typos@v1.29.5
      with:
        files: ./file.txt
        config: ./myconfig.toml

    - name: Ignore implicit configuration file
      uses: crate-ci/typos@v1.29.5
      with:
        files: ./file.txt
        isolated: true

    - name: Writes changes in the local checkout
      uses: crate-ci/typos@v1.29.5
      with:
        write_changes: true
```
