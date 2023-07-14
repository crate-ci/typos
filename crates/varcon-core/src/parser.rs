use winnow::prelude::*;
use winnow::trace::trace;

use crate::*;

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClusterIter<'i> {
    input: &'i str,
}

impl<'i> ClusterIter<'i> {
    pub fn new(input: &'i str) -> Self {
        Self { input }
    }
}

impl<'i> Iterator for ClusterIter<'i> {
    type Item = Cluster;

    fn next(&mut self) -> Option<Cluster> {
        self.input = self.input.trim_start();
        Cluster::parse_.parse_next(&mut self.input).ok()
    }
}

#[cfg(test)]
mod test_cluster_iter {
    use super::*;

    #[test]
    fn test_single() {
        let iter = ClusterIter::new(
            "# acknowledgment <verified> (level 35)
A Cv: acknowledgment / Av B C: acknowledgement
A Cv: acknowledgments / Av B C: acknowledgements
A Cv: acknowledgment's / Av B C: acknowledgement's

",
        );
        assert_eq!(iter.count(), 1);
    }

    #[test]
    fn test_multiple() {
        let iter = ClusterIter::new(
            "# acknowledgment <verified> (level 35)
A Cv: acknowledgment / Av B C: acknowledgement
A Cv: acknowledgments / Av B C: acknowledgements
A Cv: acknowledgment's / Av B C: acknowledgement's

# acknowledgment <verified> (level 35)
A Cv: acknowledgment / Av B C: acknowledgement
A Cv: acknowledgments / Av B C: acknowledgements
A Cv: acknowledgment's / Av B C: acknowledgement's

",
        );
        assert_eq!(iter.count(), 2);
    }
}

impl Cluster {
    pub fn parse(input: &str) -> Result<Self, ParseError> {
        Self::parse_.parse(input).map_err(|_err| ParseError)
    }

    fn parse_(input: &mut &str) -> PResult<Self, ()> {
        trace("cluster", move |input: &mut &str| {
            let header = (
                "#",
                winnow::ascii::space0,
                winnow::ascii::not_line_ending,
                winnow::ascii::line_ending,
            );
            let note = winnow::combinator::preceded(
                ("##", winnow::ascii::space0),
                winnow::combinator::terminated(
                    winnow::ascii::not_line_ending,
                    winnow::ascii::line_ending,
                ),
            );
            let mut cluster = (
                winnow::combinator::opt(header),
                winnow::combinator::repeat(
                    1..,
                    winnow::combinator::terminated(Entry::parse_, winnow::ascii::line_ending),
                ),
                winnow::combinator::repeat(0.., note),
            );
            let (header, entries, notes): (_, _, Vec<_>) = cluster.parse_next(input)?;

            let header = header.map(|s| s.2.to_owned());
            let notes = notes.into_iter().map(|s| s.to_owned()).collect();
            let c = Self {
                header,
                entries,
                notes,
            };
            Ok(c)
        })
        .parse_next(input)
    }
}

#[cfg(test)]
mod test_cluster {
    use super::*;

    #[test]
    fn test_basic() {
        let (input, actual) = Cluster::parse_
            .parse_peek(
                "# acknowledgment <verified> (level 35)
A Cv: acknowledgment / Av B C: acknowledgement
A Cv: acknowledgments / Av B C: acknowledgements
A Cv: acknowledgment's / Av B C: acknowledgement's

",
            )
            .unwrap();
        assert_eq!(input, "\n");
        assert_eq!(
            actual.header,
            Some("acknowledgment <verified> (level 35)".to_owned())
        );
        assert_eq!(actual.entries.len(), 3);
        assert_eq!(actual.notes.len(), 0);
    }

    #[test]
    fn test_notes() {
        let (input, actual) = Cluster::parse_
            .parse_peek(
                "# coloration <verified> (level 50)
A B C: coloration / B. Cv: colouration
A B C: colorations / B. Cv: colourations
A B C: coloration's / B. Cv: colouration's
## OED has coloration as the preferred spelling and discolouration as a
## variant for British Engl or some reason

",
            )
            .unwrap();
        assert_eq!(input, "\n");
        assert_eq!(
            actual.header,
            Some("coloration <verified> (level 50)".to_owned())
        );
        assert_eq!(actual.entries.len(), 3);
        assert_eq!(actual.notes.len(), 2);
    }
}

impl Entry {
    pub fn parse(input: &str) -> Result<Self, ParseError> {
        Self::parse_.parse(input).map_err(|_err| ParseError)
    }

    fn parse_(input: &mut &str) -> PResult<Self, ()> {
        trace("entry", move |input: &mut &str| {
            let var_sep = (winnow::ascii::space0, '/', winnow::ascii::space0);
            let variants =
                winnow::combinator::separated1(Variant::parse_, var_sep).parse_next(input)?;

            let desc_sep = (winnow::ascii::space0, '|');
            let description =
                winnow::combinator::opt((desc_sep, Self::parse_description)).parse_next(input)?;

            let comment_sep = (winnow::ascii::space0, '#');
            let comment = winnow::combinator::opt((
                comment_sep,
                winnow::ascii::space1,
                winnow::ascii::not_line_ending,
            ))
            .parse_next(input)?;

            let mut e = match description {
                Some((_, description)) => description,
                None => Self {
                    variants: Vec::new(),
                    pos: None,
                    archaic: false,
                    note: false,
                    description: None,
                    comment: None,
                },
            };
            e.variants = variants;
            e.comment = comment.map(|c| c.2.to_owned());
            Ok(e)
        })
        .parse_next(input)
    }

    fn parse_description(input: &mut &str) -> PResult<Self, ()> {
        trace("description", move |input: &mut &str| {
            let (pos, archaic, note, description) = (
                winnow::combinator::opt((winnow::ascii::space1, Pos::parse_)),
                winnow::combinator::opt((winnow::ascii::space1, "(-)")),
                winnow::combinator::opt((winnow::ascii::space1, "--")),
                winnow::combinator::opt((
                    winnow::ascii::space1,
                    winnow::token::take_till0(('\n', '\r', '#')),
                )),
            )
                .parse_next(input)?;

            let variants = Vec::new();
            let pos = pos.map(|(_, p)| p);
            let archaic = archaic.is_some();
            let note = note.is_some();
            let description = description.map(|(_, d)| d.to_owned());
            let e = Self {
                variants,
                pos,
                archaic,
                note,
                description,
                comment: None,
            };
            Ok(e)
        })
        .parse_next(input)
    }
}

#[cfg(test)]
mod test_entry {
    #![allow(clippy::bool_assert_comparison)]
    use super::*;

    #[test]
    fn test_variant_only() {
        // Having nothing after `A` causes an incomplete parse. Shouldn't be a problem for my use
        // cases.
        let (input, actual) = Entry::parse_
            .parse_peek("A Cv: acknowledgment's / Av B C: acknowledgement's\n")
            .unwrap();
        assert_eq!(input, "\n");
        assert_eq!(actual.variants.len(), 2);
        assert_eq!(actual.pos, None);
        assert_eq!(actual.archaic, false);
        assert_eq!(actual.note, false);
        assert_eq!(actual.description, None);
    }

    #[test]
    fn test_description() {
        // Having nothing after `A` causes an incomplete parse. Shouldn't be a problem for my use
        // cases.
        let (input, actual) = Entry::parse_
            .parse_peek("A C: prize / B: prise | otherwise\n")
            .unwrap();
        assert_eq!(input, "\n");
        assert_eq!(actual.variants.len(), 2);
        assert_eq!(actual.pos, None);
        assert_eq!(actual.archaic, false);
        assert_eq!(actual.note, false);
        assert_eq!(actual.description, Some("otherwise".to_owned()));
    }

    #[test]
    fn test_pos() {
        // Having nothing after `A` causes an incomplete parse. Shouldn't be a problem for my use
        // cases.
        let (input, actual) = Entry::parse_
            .parse_peek("A B C: practice / AV Cv: practise | <N>\n")
            .unwrap();
        assert_eq!(input, "\n");
        assert_eq!(actual.variants.len(), 2);
        assert_eq!(actual.pos, Some(Pos::Noun));
        assert_eq!(actual.archaic, false);
        assert_eq!(actual.note, false);
        assert_eq!(actual.description, None);
    }

    #[test]
    fn test_archaic() {
        // Having nothing after `A` causes an incomplete parse. Shouldn't be a problem for my use
        // cases.
        let (input, actual) = Entry::parse_
            .parse_peek("A: bark / Av B: barque | (-) ship\n")
            .unwrap();
        assert_eq!(input, "\n");
        assert_eq!(actual.variants.len(), 2);
        assert_eq!(actual.pos, None);
        assert_eq!(actual.archaic, true);
        assert_eq!(actual.note, false);
        assert_eq!(actual.description, Some("ship".to_owned()));
    }

    #[test]
    fn test_note() {
        // Having nothing after `A` causes an incomplete parse. Shouldn't be a problem for my use
        // cases.
        let (input, actual) = Entry::parse_
            .parse_peek("_: cabbies | -- plural\n")
            .unwrap();
        assert_eq!(input, "\n");
        assert_eq!(actual.variants.len(), 1);
        assert_eq!(actual.pos, None);
        assert_eq!(actual.archaic, false);
        assert_eq!(actual.note, true);
        assert_eq!(actual.description, Some("plural".to_owned()));
    }

    #[test]
    fn test_trailing_comment() {
        let (input, actual) = Entry::parse_.parse_peek(
            "A B: accursed / AV B-: accurst # ODE: archaic, M-W: 'or' but can find little evidence of use\n",
        )
        .unwrap();
        assert_eq!(input, "\n");
        assert_eq!(actual.variants.len(), 2);
        assert_eq!(actual.pos, None);
        assert_eq!(actual.archaic, false);
        assert_eq!(actual.note, false);
        assert_eq!(actual.description, None);
        assert_eq!(
            actual.comment,
            Some("ODE: archaic, M-W: 'or' but can find little evidence of use".to_owned())
        );
    }
}

impl Variant {
    pub fn parse(input: &str) -> Result<Self, ParseError> {
        Self::parse_.parse(input).map_err(|_err| ParseError)
    }

    fn parse_(input: &mut &str) -> PResult<Self, ()> {
        trace("variant", move |input: &mut &str| {
            let types = winnow::combinator::separated1(Type::parse_, winnow::ascii::space1);
            let sep = (":", winnow::ascii::space0);
            let (types, word) =
                winnow::combinator::separated_pair(types, sep, word).parse_next(input)?;
            let v = Self { types, word };
            Ok(v)
        })
        .parse_next(input)
    }
}

fn word(input: &mut &str) -> PResult<String, ()> {
    trace("word", move |input: &mut &str| {
        winnow::token::take_till1(|item: char| item.is_ascii_whitespace())
            .map(|s: &str| s.to_owned().replace('_', " "))
            .parse_next(input)
    })
    .parse_next(input)
}

#[cfg(test)]
mod test_variant {
    use super::*;

    #[test]
    fn test_valid() {
        // Having nothing after `A` causes an incomplete parse. Shouldn't be a problem for my use
        // cases.
        let (input, actual) = Variant::parse_.parse_peek("A Cv: acknowledgment ").unwrap();
        assert_eq!(input, " ");
        assert_eq!(
            actual.types,
            vec![
                Type {
                    category: Category::American,
                    tag: None,
                    num: None,
                },
                Type {
                    category: Category::Canadian,
                    tag: Some(Tag::Variant),
                    num: None,
                }
            ]
        );
        assert_eq!(actual.word, "acknowledgment");
    }

    #[test]
    fn test_extra() {
        let (input, actual) = Variant::parse_
            .parse_peek("A Cv: acknowledgment's / Av B C: acknowledgement's")
            .unwrap();
        assert_eq!(input, " / Av B C: acknowledgement's");
        assert_eq!(
            actual.types,
            vec![
                Type {
                    category: Category::American,
                    tag: None,
                    num: None,
                },
                Type {
                    category: Category::Canadian,
                    tag: Some(Tag::Variant),
                    num: None,
                }
            ]
        );
        assert_eq!(actual.word, "acknowledgment's");
    }

    #[test]
    fn test_underscore() {
        let (input, actual) = Variant::parse_.parse_peek("_: air_gun\n").unwrap();
        assert_eq!(input, "\n");
        assert_eq!(
            actual.types,
            vec![Type {
                category: Category::Other,
                tag: None,
                num: None,
            },]
        );
        assert_eq!(actual.word, "air gun");
    }
}

impl Type {
    pub fn parse(input: &str) -> Result<Self, ParseError> {
        Self::parse_.parse(input).map_err(|_err| ParseError)
    }

    fn parse_(input: &mut &str) -> PResult<Type, ()> {
        trace("type", move |input: &mut &str| {
            let category = Category::parse_(input)?;
            let tag = winnow::combinator::opt(Tag::parse_).parse_next(input)?;
            let num = winnow::combinator::opt(winnow::ascii::digit1).parse_next(input)?;
            let num = num.map(|s| s.parse().expect("parser ensured its a number"));
            let t = Type { category, tag, num };
            Ok(t)
        })
        .parse_next(input)
    }
}

#[cfg(test)]
mod test_type {
    use super::*;

    #[test]
    fn test_valid() {
        // Having nothing after `A` causes an incomplete parse. Shouldn't be a problem for my use
        // cases.
        let (input, actual) = Type::parse_.parse_peek("A ").unwrap();
        assert_eq!(input, " ");
        assert_eq!(actual.category, Category::American);
        assert_eq!(actual.tag, None);
        assert_eq!(actual.num, None);

        let (input, actual) = Type::parse_.parse_peek("Bv ").unwrap();
        assert_eq!(input, " ");
        assert_eq!(actual.category, Category::BritishIse);
        assert_eq!(actual.tag, Some(Tag::Variant));
        assert_eq!(actual.num, None);
    }

    #[test]
    fn test_extra() {
        let (input, actual) = Type::parse_.parse_peek("Z foobar").unwrap();
        assert_eq!(input, " foobar");
        assert_eq!(actual.category, Category::BritishIze);
        assert_eq!(actual.tag, None);
        assert_eq!(actual.num, None);

        let (input, actual) = Type::parse_.parse_peek("C- foobar").unwrap();
        assert_eq!(input, " foobar");
        assert_eq!(actual.category, Category::Canadian);
        assert_eq!(actual.tag, Some(Tag::Possible));
        assert_eq!(actual.num, None);
    }

    #[test]
    fn test_num() {
        let (input, actual) = Type::parse_.parse_peek("Av1 ").unwrap();
        assert_eq!(input, " ");
        assert_eq!(actual.category, Category::American);
        assert_eq!(actual.tag, Some(Tag::Variant));
        assert_eq!(actual.num, Some(1));
    }
}

impl Category {
    pub fn parse(input: &str) -> Result<Self, ParseError> {
        Self::parse_.parse(input).map_err(|_err| ParseError)
    }

    fn parse_(input: &mut &str) -> PResult<Self, ()> {
        trace("category", move |input: &mut &str| {
            let symbols = winnow::token::one_of(['A', 'B', 'Z', 'C', 'D', '_']);
            symbols
                .map(|c| match c {
                    'A' => Category::American,
                    'B' => Category::BritishIse,
                    'Z' => Category::BritishIze,
                    'C' => Category::Canadian,
                    'D' => Category::Australian,
                    '_' => Category::Other,
                    _ => unreachable!("parser won't select this option"),
                })
                .parse_next(input)
        })
        .parse_next(input)
    }
}

#[cfg(test)]
mod test_category {
    use super::*;

    #[test]
    fn test_valid() {
        let (input, actual) = Category::parse_.parse_peek("A").unwrap();
        assert_eq!(input, "");
        assert_eq!(actual, Category::American);
    }

    #[test]
    fn test_extra() {
        let (input, actual) = Category::parse_.parse_peek("_ foobar").unwrap();
        assert_eq!(input, " foobar");
        assert_eq!(actual, Category::Other);
    }
}

impl Tag {
    pub fn parse(input: &str) -> Result<Self, ParseError> {
        Self::parse_.parse(input).map_err(|_err| ParseError)
    }

    fn parse_(input: &mut &str) -> PResult<Self, ()> {
        trace("tag", move |input: &mut &str| {
            let symbols = winnow::token::one_of(['.', 'v', 'V', '-', 'x']);
            symbols
                .map(|c| match c {
                    '.' => Tag::Eq,
                    'v' => Tag::Variant,
                    'V' => Tag::Seldom,
                    '-' => Tag::Possible,
                    'x' => Tag::Improper,
                    _ => unreachable!("parser won't select this option"),
                })
                .parse_next(input)
        })
        .parse_next(input)
    }
}

#[cfg(test)]
mod test_tag {
    use super::*;

    #[test]
    fn test_valid() {
        let (input, actual) = Tag::parse_.parse_peek(".").unwrap();
        assert_eq!(input, "");
        assert_eq!(actual, Tag::Eq);
    }

    #[test]
    fn test_extra() {
        let (input, actual) = Tag::parse_.parse_peek("x foobar").unwrap();
        assert_eq!(input, " foobar");
        assert_eq!(actual, Tag::Improper);
    }
}

impl Pos {
    pub fn parse(input: &str) -> Result<Self, ParseError> {
        Self::parse_.parse(input).map_err(|_err| ParseError)
    }

    fn parse_(input: &mut &str) -> PResult<Self, ()> {
        trace("pos", move |input: &mut &str| {
            winnow::combinator::alt((
                "<N>".value(Pos::Noun),
                "<V>".value(Pos::Verb),
                "<Adj>".value(Pos::Adjective),
                "<Adv>".value(Pos::Adverb),
            ))
            .parse_next(input)
        })
        .parse_next(input)
    }
}

#[cfg(test)]
mod test_pos {
    use super::*;

    #[test]
    fn test_valid() {
        let (input, actual) = Pos::parse_.parse_peek("<N>").unwrap();
        assert_eq!(input, "");
        assert_eq!(actual, Pos::Noun);
    }

    #[test]
    fn test_extra() {
        let (input, actual) = Pos::parse_.parse_peek("<Adj> foobar").unwrap();
        assert_eq!(input, " foobar");
        assert_eq!(actual, Pos::Adjective);
    }
}

#[derive(Debug)]
pub struct ParseError;

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "invalid")
    }
}

impl std::error::Error for ParseError {}
