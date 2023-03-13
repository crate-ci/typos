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
- Machine-independent, repo-specific configuration
  - As compared to layered config with the users system or the command-line

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

## Identifiers and Words

With a focus on spell checking source code, most text will be in the form of
identifiers that are made up of words conjoined via `snake_case`, `CamelCase`,
etc.  A typo at the word level might not be a typo as part of
an identifier, so identifiers get checked and, if not in a dictionary, will
then be split into words to be checked.

Identifiers are defined using
[unicode's `XID_Continue`](https://www.unicode.org/reports/tr31/#Table_Lexical_Classes_for_Identifiers)
which includes `[a-zA-Z0-9_]`.

Words are split from identifiers on case changes as well as breaks in
`[a-zA-Z]` with a special case to handle acronyms.  For example,
`First10HTMLTokens` would be split as `first`, `html`, `tokens`.

To see this in action, run `typos --identifiers` or `typos --words`.
