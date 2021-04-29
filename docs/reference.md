# `typos` Reference

## Configuration

### Sources

Configuration is read from the following (in precedence order)

- Command line arguments
- File specified via `--config PATH`
- Search parents of specified file / directory for one of `typos.toml`, `_typos.toml`, or `.typos.toml`

### Config Fields

| Field                  | Argument          | Format | Description |
|------------------------|-------------------|--------|-------------|
| files.ignore-hidden    | --hidden          | bool   | Skip hidden files and directories. |
| files.ignore-files     | --ignore          | bool   | Respect ignore files. |
| files.ignore-dot       | --ignore-dot      | bool   | Respect .ignore files. |
| files.ignore-vcs       | --ignore-vcs      | bool   | Respect ignore files in vcs directories. |
| files.ignore-global    | --ignore-global   | bool   | Respect global ignore files. |
| files.ignore-parent    | --ignore-parent   | bool   | Respect ignore files in parent directories. |
| default.binary         | --binary          | bool   | Check binary files as text |
| default.check-filename | \-                | bool   | Verifying spelling in file names. |
| default.check-file     | \-                | bool   | Verifying spelling in files. |
| default.unicode        | \-                | bool   | Allow unicode characters in identifiers (and not just ASCII) |
| default.ignore-hex     | \-                | bool   | Do not check identifiers that appear to be hexadecimal values. |
| default.identifier-leading-digits   | \-   | bool   | Allow identifiers to start with digits, in addition to letters. |
| default.locale         | --locale          | en, en-us, en-gb, en-ca, en-au   | English dialect to correct to. |
| default.extend-identifiers | \-            | table of strings | Corrections for identifiers. When the correction is blank, the word is never valid. When the correction is the key, the word is always valid. |
| default.extend-words       | \-            | table of strings | Corrections for identifiers. When the correction is blank, the word is never valid. When the correction is the key, the word is always valid. |
| type.<name>.<field>        | <varied>      | <varied>   | See `default.` for child keys.  Run with `--type-list` to see available `<name>`s |
| type.<name>.extend_globs   | \-            | list of strings  | File globs for matching `<name>` |
