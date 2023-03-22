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
| files.binary           | --binary          | bool   | Check binary files as text |
| files.extend-exclude   | --exclude         | list of strings | Typos-specific ignore globs (gitignore syntax) |
| files.ignore-hidden    | --hidden          | bool   | Skip hidden files and directories. |
| files.ignore-files     | --ignore          | bool   | Respect ignore files. |
| files.ignore-dot       | --ignore-dot      | bool   | Respect .ignore files. |
| files.ignore-vcs       | --ignore-vcs      | bool   | Respect ignore files in vcs directories. |
| files.ignore-global    | --ignore-global   | bool   | Respect global ignore files. |
| files.ignore-parent    | --ignore-parent   | bool   | Respect ignore files in parent directories. |
| default.binary         | --binary          | bool   | Check binary files as text |
| default.check-filename | \-                | bool   | Verifying spelling in file names. |
| default.check-file     | \-                | bool   | Verifying spelling in files. |
| default.unicode        | --unicode         | bool   | Allow unicode characters in identifiers (and not just ASCII) |
| default.locale         | --locale          | en, en-us, en-gb, en-ca, en-au   | English dialect to correct to. |
| default.extend-identifiers | \-            | table of strings | Corrections for [identifiers](./design.md#identifiers-and-words). When the correction is blank, the identifier is never valid. When the correction is the key, the identifier is always valid. |
| default.extend-ignore-identifiers-re | \-            | list of [regexes](https://docs.rs/regex/latest/regex/index.html#syntax) | Pattern-match always-valid identifiers |
| default.extend-words       | \-            | table of strings | Corrections for [words](./design.md#identifiers-and-words). When the correction is blank, the word is never valid. When the correction is the key, the word is always valid. |
| type.\<name>.\<field>      | \<varied>     | \<varied>  | See `default.` for child keys.  Run with `--type-list` to see available `<name>`s |
| type.\<name>.extend-glob   | \-            | list of strings  | File globs for matching `<name>` |
