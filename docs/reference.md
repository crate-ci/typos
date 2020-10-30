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
| files.ignore-patterns  |                   | list of strings | Typos-specific ignore globs (gitignore syntax) |
| files.ignore-hidden    | --hidden          | bool   | Skip hidden files and directories. |
| files.ignore-files     | --ignore          | bool   | Respect ignore files. |
| files.ignore-dot       | --ignore-dot      | bool   | Respect .ignore files. |
| files.ignore-vcs       | --ignore-vcs      | bool   | Respect ignore files in vcs directories. |
| files.ignore-global    | --ignore-global   | bool   | Respect global ignore files. |
| files.ignore-parent    | --ignore-parent   | bool   | Respect ignore files in parent directories. |
| default.check-filename | \-                | bool   | Verifying spelling in file names. |
| default.check-file     | \-                | bool   | Verifying spelling in files. |
| default.ignore-hex     | \-                | bool   | Do not check identifiers that appear to be hexadecimal values. |
| default.identifier-leading-digits   | \-   | bool   | Allow identifiers to start with digits, in addition to letters. |
| default.identifier-include-digits   | \-   | bool   | Allow identifiers to include digits, in addition to letters. |
| default.identifier-leading-chars    | \-   | string | Allow identifiers to start with one of these characters. |
| default.identifier-include-chars    | \-   | string | Allow identifiers to include these characters. |
| default.locale         | \-                | en, en-us, en-gb, en-ca, en-au   | English dialect to correct to. |
| default.extend-identifiers | \-            | table of strings | Corrections for identifiers. When the correction is blank, the word is never valid. When the correction is the key, the word is always valid. |
| default.extend-words       | \-            | table of strings | Corrections for identifiers. When the correction is blank, the word is never valid. When the correction is the key, the word is always valid. |
