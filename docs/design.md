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

Dictionary: A confidence rating is given for how close a word is to one in a dictionary
- Sensitive to false positives due to hex numbers and c-escapes
- Traditional spell checkers use a dictionary.

