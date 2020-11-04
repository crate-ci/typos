# About `typos`

## Design Requirements

Spell checks source code:
- Requires special word-splitting logic to handle situations like hex (`0xDEADBEEF`), `c\nescapes`, `snake_case`, `CamelCase`, `SCREAMING_CASE`, and maybe `arrow-case`.
- Each programming language has its own quirks, like abbreviations, lack of word separator (`copysign`), etc
- Backwards compatibility might require keeping misspelled words.
- Case for proper nouns is irrelevant.

Checking for errors in a CI:
- No false-positives.
- On spelling errors, sets the exit code to fail the CI.

Quick feedback and resolution for developer:
- Fix errors for the user.
- Integration into other programs, like editors:
  - `fork`: easy to call into and provides a stable API, including output format
  - linking: either in the language of choice or bindings can be made to language of choice.

## Design Trade Offs

### typos uses a blacklist

Blacklist: Known typos that map to their corresponding word
- Ignores unknown typos
- Ignores typos that follow c-escapes if they aren't handled correctly

Whitelist: A confidence rating is given for how close a word is to one in the whitelist
- Sensitive to false positives due to hex numbers and c-escapes
- Traditional spell checkers use a whitelist.

## Related Spell Checkers

See also [benchmarks](../benchsuite/runs).

|                | typos                 | [bloom42/misspell][misspell-rs] | [client9/misspell][misspell-go] | [codespell] | [scspell3k] |
|----------------|-----------------------|---------------------------------|---------------------------------|-------------|-------------|
| Runtime        | \-                    | \-                              | \-                              | Python      | Python      |
| Dictionary     | Blacklist             | Blacklist                       | Blacklist                       | Blacklist   | Whitelist   |
| Custom Dict    | Yes                   | No                              | ?                               | Yes         | Yes         |
| Per-Lang Dict  | No ([#14][def-14])    | No                              | ?                               | No          | Yes         |
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
