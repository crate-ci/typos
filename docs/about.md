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

|               | typos                 | [scspell3k] | [bloom42/misspell][misspell-rs] | [client9/misspell][misspell-go] |
|---------------|-----------------------|-------------|---------------------------------|---------------------------------|
| "Runtime"     | Rust ([#18][def-18])  | Python      | Rust                            | None                            |
| Dictionary    | Blacklist             | Whitelist   | Blacklist                       | Blacklist                       |
| Custom Dict   | No ([#9][def-9])      | Yes         | No                              | ?                               |
| Per-Lang Dict | No ([#14][def-14])    | Yes         | No                              | ?                               |
| CamelCase     | Yes                   | Yes         | No                              | ?                               |
| snake_case    | Yes                   | Yes         | No                              | ?                               |
| Ignore Hex    | No ([#19][def-19])    | Yes         | No                              | ?                               |
| C-Escapes     | No ([#20][def-3])     | Yes         | No                              | ?                               |
| Encodings     | UTF-8 ([#17][def-17]) | Auto        | UTF-8                           | ?                               |
| API           | Rust / [JSON Lines]   | None        | Rust                            | ?                               |
| License       | MIT or Apache         | GPLv2       | AGPL                            | MIT                             |

[JSON Lines]: http://jsonlines.org/
[scspell3k]: https://github.com/myint/scspell
[misspell-rs]: https://gitlab.com/bloom42/misspell
[misspell-go]: https://github.com/client9/misspell
[def-9]: https://github.com/epage/typos/issues/9
[def-14]: https://github.com/epage/typos/issues/14
[def-17]: https://github.com/epage/typos/issues/17
[def-18]: https://github.com/epage/typos/issues/18
[def-19]: https://github.com/epage/typos/issues/19
[def-3]: https://github.com/epage/typos/issues/3
