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

### Format

Summary of configuration (see below for details)
```toml
[files]
extend-exclude = []
ignore-hidden = true
ignore-files = true
ignore-dot = true
ignore-vcs = true
ignore-global = true
ignore-parent = true

[default]
binary = false
check-filename = true
check-file = true
unicode=true
locale = "en"
extend-ignore-re = []
extend-ignore-identifiers-re = []
extend-ignore-words-re = []

[default.extend-words]
# <typo> = "<correction>"

[default.extend-identifiers]
# <typo> = "<correction>"

[type.NAME]
extend-glob = []
# ... see `default`
```

Notes:
- For the distinction between "words" and "identifiers", see [design](design.md#identifiers-and-words)

### Configuration keys

#### `files.extend-exclude`

- Type: list of strings
- CLI: `--exclude`

Typos-specific ignore globs (gitignore syntax)

Note: the command-line overrides this by default.
See `--force-exclude` to ensure this field is always respected.

Example of an include list:
```toml
[files]
extend-exclude = [
  "*",
  "!something",
]
```

#### `files.ignore-hidden`

- Type: bool
- Default: true
- CLI: --hidden

Skip hidden files and directories.

#### `files.ignore-files`

- Type: bool
- Default: true
- CLI: --ignore

Respect ignore files.

#### `files.ignore-dot`

- Type: bool
- Default: true
- CLI: `--ignore-dot`

Respect .ignore files.

#### `files.ignore-vcs`
- Type: bool
- Default: true
- CLI: `--ignore-vcs`

Respect ignore files in vcs directories.

#### `files.ignore-global`

- Type: bool
- Default: true
- CLI: `--ignore-global`

Respect global ignore files.

#### `files.ignore-parent`

- Type: bool
- Default: true
- CLI: `--ignore-parent`

Respect ignore files in parent directories.

#### `default.binary`

- Type: bool
- Default: false
- CLI: `--binary`

Check binary files as text.

#### `default.check-filename`

- Type: bool
- Default: true

Verify spelling in file names.

Directory names are not checked.

#### `default.check-file`

- Type: bool
- Default: true

Verify spelling in files.

#### `default.unicode`

- Type: bool
- Default: true
- CLI: `--unicode`

Allow unicode characters in identifiers (and not just ASCII).

#### `default.locale`

- Type: String (`en`, `en-us`, `en-gb`, `en-ca`, `en-au`)
- Default: `en`
- CLI: `--locale`

English dialect to correct to.

If set to `en`,
words will be corrected to the closest spelling,
regardless of which dialect that correction is part of.

#### `default.extend-ignore-re`

- Type: list of [regexes](https://docs.rs/regex/latest/regex/index.html#syntax)

Custom uncorrectable sections (e.g. markdown code fences, PGP signatures, etc)

#### `default.extend-identifiers`

- Type: table of strings

Map [identifier](./design.md#identifiers-and-words) typos to their corrections.
When the correction is blank, the identifier is never valid.
When the correction is the key, the identifier is always valid.

Example:
```toml
[default.extend-identifiers]
## Names
Hte = "Hte"
## External
ERROR_FILENAME_EXCED_RANGE = "ERROR_FILENAME_EXCED_RANGE"
ERROR_FILENAME_EXCEDE_RANGE = "ERROR_FILENAME_EXCED_RANGE"
```

#### `default.extend-ignore-identifiers-re`

- Type: list of [regexes](https://docs.rs/regex/latest/regex/index.html#syntax)

Pattern-match always-valid identifiers.

#### `default.extend-words`

- Type: table of strings

Map [word](./design.md#identifiers-and-words) typos to their corrections.
When the correction is blank, the word is never valid.
When the correction is the key, the word is always valid.

Example:
```toml
[default.extend-words]
## Project-specific acronym
taits = "taits"
tais = "taits"
```

#### `default.extend-ignore-words-re`

- Type: list of [regexes](https://docs.rs/regex/latest/regex/index.html#syntax)

Pattern-match always-valid words.  Note: you must handle case insensitivity yourself.

#### `type.NAME.extend-glob`

- Type: list of strings

File globs for matching `NAME`. This is required when defining new file types.

When there are multiple globs that would match,
the most specific glob is used.

Run with `--type-list` to see available `NAME`s.

### Example configurations

Common `extend-ignore-re`:
- Line ignore with trailing `# spellchecker:disable-line`: `"(?Rm)^.*(#|//)\\s*spellchecker:disable-line$"`
- Line block with `# spellchecker:<on|off>`: `"(?s)(#|//)\\s*spellchecker:off.*?\\n\\s*(#|//)\\s*spellchecker:on"`
- Next-line ignore `# spellchecker:ignore-next-line`: `(#|//)\\s*spellchecker:ignore-next-line\\n.*`
- See also [ripsecret's regexes](https://github.com/sirwart/ripsecrets/blob/main/src/lib.rs)

Common `extend-ignore-identifiers-re`:
- SSL Cipher suites: `"\\bTLS_[A-Z0-9_]+(_anon_[A-Z0-9_]+)?\\b"`
