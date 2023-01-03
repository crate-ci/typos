# Design

## Requirements

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

## Trade Offs

### Corrections vs Dictionaries

Corrections: Known misspellings that map to their corresponding dictionary word
- Ignores unknown typos
- Ignores typos that follow c-escapes if they aren't handled correctly
- Good for unassisted automated correcting
- Fast, can quickly run across large code bases

Dictionary: A confidence rating is given for how close a word is to one in a dictionary
- Sensitive to false positives due to hex numbers and c-escapes
- Used in word processors and other traditional spell checking applications
- Good when there is a UI to let the user know and override any decisions

## Words vs Identifiers

`typos` reuses Unicode's 'word' and 'identifier' definitions, see in particular [unicode's `XID_Continue`](https://www.unicode.org/reports/tr31/#Table_Lexical_Classes_for_Identifiers).
The Unicode standard is hard to read, thus here's an incomplete TL;DR that should cover the most common cases for `typos`.

From any text such as `my_string = "Hello TyposWorld"`, identifiers (sequences of words) and words are relevant sequences of characters for our purposes.
Characters such as spaces separate identifiers from each other, in the example `my_string`, `Hello`, and `TyposWorld`.
In addition to identifier separators, characters such as underscore are uppercase characters separate words from each other, in the example `my`, `string`, `Hello`, `Typos`, and `World`.

The specific rules are:

- A word is continued when the next character matches `[a-z0-9]`
- An identifier is continued when the next character matches `[A-Z_]`
- Other characters separate identifiers and hence words.
