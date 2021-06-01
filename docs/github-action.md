# GitHub Action

If you want an easy way to test your repository spelling (or a subset of files)
you can use the Typos Action! It is served from this repository, and can
easily be used as follows:

```yaml
name: Test GitHub Action
on: [pull_request]

jobs:
  run:
    name: Spell Check with Typos
    runs-on: ubuntu-latest
    steps:
    - name: Checkout Actions Repository
      uses: actions/checkout@v2

    - name: Check spelling of file.txt
      uses: crate-ci/typos@main
      with: 
        files: ./file.txt

    - name: Use custom config file
      uses: crate-ci/typos@main
      with: 
        files: ./file.txt
        config: ./myconfig.toml

    - name: Ignore implicit configuration file
      uses: crate-ci/typos@main
      with: 
        files: ./file.txt
        isolated: true
```

**Important** for any of the examples above, make sure that you choose
a release or commit as a version, and not a branch (which is a moving target).
Also make sure when referencing relative file paths to use `./` (e.g., `./file.txt` instead of
`file.txt`.

## Variables

| Name | Description | Required | Default |
|------|-------------|----------|---------|
| files| Files or patterns to check | false | If not defined, entire repository is checked |
| isolated | Ignore implicit configuration files | false | false|
| config | Use a custom config file (must exist) | false | not set |
