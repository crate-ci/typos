# Related Spell Checkers

|                | typos                 | [golangci/misspell][misspell]   | [codespell] | [scspell3k] | [topy]     |
|----------------|-----------------------|---------------------------------|-------------|-------------|------------|
| Runtime        | \-                    | \-                              | Python      | Python      | Python     |
| [Approach](design.md) | Correction     | Correction                      | Correction  | Dictionary  | Dictionary |
| Custom Dict    | Yes                   | Yes                             | Yes         | Yes         | Yes        |
| Per-Lang Dict  | Yes                   | No                              | No          | Yes         | No         |
| CamelCase      | Yes                   | No                              | No          | Yes         | No         |
| snake_case     | Yes                   | Yes                             | No          | Yes         | No         |
| Ignore email   | Yes                   | yes                             | No          | No          | No         |
| Ignore url     | Yes                   | yes                             | No          | No          | No         |
| Ignore Hex     | Yes                   | No                              | No          | Yes         | No         |
| Ignore UUID    | Yes                   | No                              | No          | No          | No         |
| Ignore base64  | Yes                   | No                              | No          | No          | No         |
| Ignore SHAs    | Yes                   | No                              | No          | No          | No         |
| C-Escapes      | Yes ([#20][def-3])    | Yes                             | No          | Yes         | No         |
| Encodings      | UTF-8 / UTF-16        | UTF-8                           | Auto        | Auto        | UTF-8      |
| Whole-project  | Yes                   | Yes                             | Yes         | No          | Yes        |
| Ignores hidden | Yes                   | No                              | Yes         | No          | Yes        |
| Respect gitignore | Yes                | No                              | No          | No          | No         |
| Checks filenames | Yes                 | No                              | Yes         | No          | No         |
| Status via exit code | Yes             | Yes                             | Yes         | Yes         | No         |
| API            | Rust / [JSON Lines]   | Go                              | Python      | None        | Python     |
| License        | MIT or Apache         | MIT                             | GPLv2       | GPLv2       | MIT        |

`misspell` filters common binary formats and SCM paths before reading files
([source][misspell-files]). Its text pass excludes emails, hostnames,
slash-delimited paths, backslash escapes, and HTTP(S)/FTP URLs
([source][misspell-notwords], [source][misspell-urls]). Go mode checks comments
rather than identifiers ([source][misspell-replace]); the CLI otherwise walks
all files and provides no hidden-file, `.gitignore`, or filename filtering
([source][misspell-cli]).

See also [benchmarks](../benchsuite/runs).

[JSON Lines]: https://jsonlines.org/
[scspell3k]: https://github.com/myint/scspell
[misspell]: https://github.com/golangci/misspell/tree/v0.8.0
[misspell-files]: https://github.com/golangci/misspell/blob/v0.8.0/mime.go
[misspell-notwords]: https://github.com/golangci/misspell/blob/v0.8.0/notwords.go
[misspell-urls]: https://github.com/golangci/misspell/blob/v0.8.0/url.go
[misspell-replace]: https://github.com/golangci/misspell/blob/v0.8.0/replace.go
[misspell-cli]: https://github.com/golangci/misspell/blob/v0.8.0/cmd/misspell/main.go
[codespell]: https://github.com/codespell-project/codespell
[topy]: https://github.com/intgr/topy
[def-9]: https://github.com/crate-ci/typos/issues/9
[def-14]: https://github.com/crate-ci/typos/issues/14
[def-17]: https://github.com/crate-ci/typos/issues/17
[def-18]: https://github.com/crate-ci/typos/issues/18
[def-3]: https://github.com/crate-ci/typos/issues/3
