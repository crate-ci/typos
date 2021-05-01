# Related Spell Checkers

|                | typos                 | [bloom42/misspell][misspell-rs] | [client9/misspell][misspell-go] | [codespell] | [scspell3k] |
|----------------|-----------------------|---------------------------------|---------------------------------|-------------|-------------|
| Runtime        | \-                    | \-                              | \-                              | Python      | Python      |
| [Approach](design.md) | Correction     | Correction                      | Correction                      | Correction  | Dictionary  |
| Custom Dict    | Yes                   | No                              | ?                               | Yes         | Yes         |
| Per-Lang Dict  | Yes                   | No                              | ?                               | No          | Yes         |
| CamelCase      | Yes                   | No                              | ?                               | No          | Yes         |
| snake_case     | Yes                   | No                              | ?                               | No          | Yes         |
| Ignore Hex     | Yes                   | No                              | ?                               | No          | Yes         |
| C-Escapes      | No ([#20][def-3])     | No                              | ?                               | No          | Yes         |
| Encodings      | UTF-8 / UTF-16        | UTF-8                           | ?                               | Auto        | Auto        |
| Whole-project  | Yes                   | Yes                             | Yes                             | Yes         | No          |
| Ignores hidden | Yes                   | Yes                             | ?                               | Yes         | No          |
| Respect gitignore | Yes                | Yes                             | ?                               | No          | No          |
| Checks filenames | Yes                 | No                              | ?                               | Yes         | No          |
| Status via exit code | Yes             | No                              | Yes                             | Yes         | Yes         |
| API            | Rust / [JSON Lines]   | Rust                            | ?                               | Python      | None        |
| License        | MIT or Apache         | AGPL                            | MIT                             | GPLv2       | GPLv2       |

See also [benchmarks](../benchsuite/runs).

[JSON Lines]: http://jsonlines.org/
[scspell3k]: https://github.com/myint/scspell
[misspell-rs]: https://gitlab.com/bloom42/misspell
[misspell-go]: https://github.com/client9/misspell
[codespell]: https://github.com/codespell-project/codespell
[def-9]: https://github.com/crate-ci/typos/issues/9
[def-14]: https://github.com/crate-ci/typos/issues/14
[def-17]: https://github.com/crate-ci/typos/issues/17
[def-18]: https://github.com/crate-ci/typos/issues/18
[def-3]: https://github.com/crate-ci/typos/issues/3
