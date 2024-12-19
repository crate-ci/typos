# `typos` Reference

## Configuration

### Sources

Configuration is read from the following (in precedence order)

- Command line arguments
- File specified via `--config PATH`
- Search parents of specified file / directory for one of `typos.toml`, `_typos.toml`, `.typos.toml`, `Cargo.toml`, or `pyproject.toml`.
  - In `pyproject.toml`, the below fields must be under the `[tool.typos]` section. If this section does not
    exist, the config file will be skipped.
  - In `Cargo.toml`, the below fields must be under either `[workspace.metadata.typos]` or `[package.metadata.typos]`

### Config Fields

| Field                  | Argument          | Format | Default | Description |
|------------------------|-------------------|--------|---------|-------------|
| files.binary           | --binary          | bool   | false   | Check binary files as text. |
| files.extend-exclude   | --exclude         | list of strings | \- | Typos-specific ignore globs (gitignore syntax). |
| \-                     | --force-exclude   | bool   | false   | Respect excluded files even for paths passed explicitly. |
| files.ignore-hidden    | --hidden          | bool   | true    | Skip hidden files and directories. |
| files.ignore-files     | --ignore          | bool   | true    | Respect ignore files. |
| files.ignore-dot       | --ignore-dot      | bool   | true    | Respect .ignore files. |
| files.ignore-vcs       | --ignore-vcs      | bool   | true    | Respect ignore files in vcs directories. |
| files.ignore-global    | --ignore-global   | bool   | true    | Respect global ignore files. |
| files.ignore-parent    | --ignore-parent   | bool   | true    | Respect ignore files in parent directories. |
| default.binary         | --binary          | bool   | false   | Check binary files as text. |
| default.check-filename | \-                | bool   | true    | Verify spelling in file names. |
| default.check-file     | \-                | bool   | true    | Verify spelling in files. |
| default.unicode        | --unicode         | bool   | true    | Allow unicode characters in identifiers (and not just ASCII). |
| default.locale         | --locale          | en, en-us, en-gb, en-ca, en-au | en | English dialect to correct to. |
| default.extend-ignore-re   | \-            | list of [regexes](https://docs.rs/regex/latest/regex/index.html#syntax) | \- | Custom uncorrectable sections (e.g. markdown code fences, PGP signatures, etc) |
| default.extend-identifiers | \-            | table of strings | \- | Corrections for [identifiers](./design.md#identifiers-and-words). When the correction is blank, the identifier is never valid. When the correction is the key, the identifier is always valid. |
| default.extend-ignore-identifiers-re | \-  | list of [regexes](https://docs.rs/regex/latest/regex/index.html#syntax) | \- | Pattern-match always-valid identifiers. |
| default.extend-words       | \-            | table of strings | \- | Corrections for [words](./design.md#identifiers-and-words). When the correction is blank, the word is never valid. When the correction is the key, the word is always valid. |
| default.extend-ignore-words-re | \-        | list of [regexes](https://docs.rs/regex/latest/regex/index.html#syntax) | \- | Pattern-match always-valid words.  Note: you must handle case insensitivity yourself. |
| type.\<name>.\<field>      | \<varied>     | \<varied> | \<varied> | See `default.` for child keys.  Run with `--type-list` to see available `<name>`s. |
| type.\<name>.extend-glob   | \-            | list of strings | \- | File globs for matching `<name>`. |

### Example configurations

Common `extend-ignore-re`:
- Line ignore with trailing `# spellchecker:disable-line`: `"(?Rm)^.*(#|//)\\s*spellchecker:disable-line$"`
- Line block with `# spellchecker:<on|off>`: `"(?s)(#|//)\\s*spellchecker:off.*?\\n\\s*(#|//)\\s*spellchecker:on"`
- See also [ripsecret's regexes](https://github.com/sirwart/ripsecrets/blob/main/src/lib.rs)

Common `extend-ignore-identifiers-re`:
- SSL Cipher suites: `"\\bTLS_[A-Z0-9_]+(_anon_[A-Z0-9_]+)?\\b"`
