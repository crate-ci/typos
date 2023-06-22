# Related Spell Checkers

|                | typos                 | [client9/misspell][misspell-go] | [codespell] | [scspell3k] | [topy]     |
|----------------|-----------------------|---------------------------------|-------------|-------------|------------|
| Runtime        | \-                    | \-                              | Python      | Python      | Python     |
| [Approach](design.md) | Correction     | Correction                      | Correction  | Dictionary  | Dictionary |
| Custom Dict    | Yes                   | ?                               | Yes         | Yes         | Yes        |
| Per-Lang Dict  | Yes                   | ?                               | No          | Yes         | No         |
| CamelCase      | Yes                   | ?                               | No          | Yes         | No         |
| snake_case     | Yes                   | ?                               | No          | Yes         | No         |
| Ignore email   | Yes                   | yes                             | No          | No          | No         |
| Ignore url     | Yes                   | yes                             | No          | No          | No         |
| Ignore Hex     | Yes                   | ?                               | No          | Yes         | No         |
| Ignore UUID    | Yes                   | ?                               | No          | No          | No         |
| Ignore base64  | Yes                   | ?                               | No          | No          | No         |
| Ignore SHAs    | Yes                   | ?                               | No          | No          | No         |
| C-Escapes      | Yes ([#20][def-3])    | ?                               | No          | Yes         | No         |
| Encodings      | UTF-8 / UTF-16        | ?                               | Auto        | Auto        | UTF-8      |
| Whole-project  | Yes                   | Yes                             | Yes         | No          | Yes        |
| Ignores hidden | Yes                   | ?                               | Yes         | No          | Yes        |
| Respect gitignore | Yes                | ?                               | No          | No          | No         |
| Checks filenames | Yes                 | ?                               | Yes         | No          | No         |
| Status via exit code | Yes             | Yes                             | Yes         | Yes         | No         |
| API            | Rust / [JSON Lines]   | ?                               | Python      | None        | Python     |
| License        | MIT or Apache         | MIT                             | GPLv2       | GPLv2       | MIT        |

See also [benchmarks](../benchsuite/runs).

[JSON Lines]: http://jsonlines.org/
[scspell3k]: https://github.com/myint/scspell
[misspell-go]: https://github.com/client9/misspell
[codespell]: https://github.com/codespell-project/codespell
[topy]: https://github.com/intgr/topy
[def-9]: https://github.com/crate-ci/typos/issues/9
[def-14]: https://github.com/crate-ci/typos/issues/14
[def-17]: https://github.com/crate-ci/typos/issues/17
[def-18]: https://github.com/crate-ci/typos/issues/18
[def-3]: https://github.com/crate-ci/typos/issues/3
