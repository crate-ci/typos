# `typos` Reference

## Configuration

### Sources

Configuration is read from the following (in precedence order)

- Command line arguments
- File specified via `--config PATH`
- Search parents of specified file / directory for `typos.toml`

### Config Fields

| Field                  | Argument          | Format | Description |
|------------------------|-------------------|--------|-------------|
| files.binary           | --binary          | bool   |  |
| files.ignore-hidden    | --hidden          | bool   |  |
| files.ignore-files     | --ignore          | bool   |  |
| files.ignore-dot       | --ignore-dot      | bool   |  |
| files.ignore-vcs       | --ignore-vcs      | bool   |  |
| files.ignore-global    | --ignore-global   | bool   |  |
| files.ignore-parent    | --ignore-parent   | bool   |  |
| default.check-filename | \-                | bool   |  |
| default.check-file     | \-                | bool   |  |
| default.ignore-hex     | \-                | bool   |  |
| default.identifier-include-digits   | \-   | bool   |  |
| default.identifier-include-chars    | \-   | string |  |
