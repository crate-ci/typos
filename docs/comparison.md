# Related Spell Checkers

|                | typos                 | [client9/misspell][misspell-go] | [codespell] | [scspell3k] |
|----------------|-----------------------|---------------------------------|-------------|-------------|
| Runtime        | \-                    | \-                              | Python      | Python      |
| [Approach](design.md) | Correction     | Correction                      | Correction  | Dictionary  |
| Custom Dict    | Yes                   | ?                               | Yes         | Yes         |
| Per-Lang Dict  | Yes                   | ?                               | No          | Yes         |
| CamelCase      | Yes                   | ?                               | No          | Yes         |
| snake_case     | Yes                   | ?                               | No          | Yes         |
| Ignore Hex     | Yes                   | ?                               | No          | Yes         |
| C-Escapes      | No ([#20][def-3])     | ?                               | No          | Yes         |
| Encodings      | UTF-8 / UTF-16        | ?                               | Auto        | Auto        |
| Whole-project  | Yes                   | Yes                             | Yes         | No          |
| Ignores hidden | Yes                   | ?                               | Yes         | No          |
| Respect gitignore | Yes                | ?                               | No          | No          |
| Checks filenames | Yes                 | ?                               | Yes         | No          |
| Status via exit code | Yes             | Yes                             | Yes         | Yes         |
| API            | Rust / [JSON Lines]   | ?                               | Python      | None        |
| License        | MIT or Apache         | MIT                             | GPLv2       | GPLv2       |

See also [benchmarks](../benchsuite/runs).

[JSON Lines]: http://jsonlines.org/
[scspell3k]: https://github.com/myint/scspell
[misspell-go]: https://github.com/client9/misspell
[codespell]: https://github.com/codespell-project/codespell
[def-9]: https://github.com/crate-ci/typos/issues/9
[def-14]: https://github.com/crate-ci/typos/issues/14
[def-17]: https://github.com/crate-ci/typos/issues/17
[def-18]: https://github.com/crate-ci/typos/issues/18
[def-3]: https://github.com/crate-ci/typos/issues/3
