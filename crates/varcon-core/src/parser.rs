use winnow::prelude::*;

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
        let i = self.input.trim_start();
        let (i, c) = Cluster::parse(i).ok()?;
        self.input = i;
        Some(c)
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
    pub fn parse(input: &str) -> IResult<&str, Self> {
        let header = (
            winnow::bytes::tag("#"),
            winnow::character::space0,
            winnow::character::not_line_ending,
            winnow::character::line_ending,
        );
        let note = winnow::sequence::preceded(
            (winnow::bytes::tag("##"), winnow::character::space0),
            winnow::sequence::terminated(
                winnow::character::not_line_ending,
                winnow::character::line_ending,
            ),
        );
        let mut cluster = (
            winnow::combinator::opt(header),
            winnow::multi::many1(winnow::sequence::terminated(
                Entry::parse,
                winnow::character::line_ending,
            )),
            winnow::multi::many0(note),
        );
        let (input, (header, entries, notes)): (_, (_, _, Vec<_>)) = cluster.parse_next(input)?;

        let header = header.map(|s| s.2.to_owned());
        let notes = notes.into_iter().map(|s| s.to_owned()).collect();
        let c = Self {
            header,
            entries,
            notes,
        };
        Ok((input, c))
    }
}

#[cfg(test)]
mod test_cluster {
    use super::*;

    #[test]
    fn test_basic() {
        let (input, actual) = Cluster::parse(
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
        let (input, actual) = Cluster::parse(
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
    pub fn parse(input: &str) -> IResult<&str, Self> {
        let var_sep = (winnow::character::space0, '/', winnow::character::space0);
        let (input, variants) =
            winnow::multi::separated1(Variant::parse, var_sep).parse_next(input)?;

        let desc_sep = (winnow::character::space0, '|');
        let (input, description) =
            winnow::combinator::opt((desc_sep, Self::parse_description)).parse_next(input)?;

        let comment_sep = (winnow::character::space0, '#');
        let (input, comment) = winnow::combinator::opt((
            comment_sep,
            winnow::character::space1,
            winnow::character::not_line_ending,
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
        Ok((input, e))
    }

    fn parse_description(input: &str) -> IResult<&str, Self> {
        let (input, (pos, archaic, note, description)) = (
            winnow::combinator::opt((winnow::character::space1, Pos::parse)),
            winnow::combinator::opt((winnow::character::space1, "(-)")),
            winnow::combinator::opt((winnow::character::space1, "--")),
            winnow::combinator::opt((
                winnow::character::space1,
                winnow::bytes::take_till0(|c| c == '\n' || c == '\r' || c == '#'),
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
        Ok((input, e))
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
        let (input, actual) =
            Entry::parse("A Cv: acknowledgment's / Av B C: acknowledgement's\n").unwrap();
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
        let (input, actual) = Entry::parse("A C: prize / B: prise | otherwise\n").unwrap();
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
        let (input, actual) = Entry::parse("A B C: practice / AV Cv: practise | <N>\n").unwrap();
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
        let (input, actual) = Entry::parse("A: bark / Av B: barque | (-) ship\n").unwrap();
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
        let (input, actual) = Entry::parse("_: cabbies | -- plural\n").unwrap();
        assert_eq!(input, "\n");
        assert_eq!(actual.variants.len(), 1);
        assert_eq!(actual.pos, None);
        assert_eq!(actual.archaic, false);
        assert_eq!(actual.note, true);
        assert_eq!(actual.description, Some("plural".to_owned()));
    }

    #[test]
    fn test_trailing_comment() {
        let (input, actual) = Entry::parse(
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
    pub fn parse(input: &str) -> IResult<&str, Self> {
        let types = winnow::multi::separated1(Type::parse, winnow::character::space1);
        let sep = (winnow::bytes::tag(":"), winnow::character::space0);
        let (input, (types, word)) =
            winnow::sequence::separated_pair(types, sep, word).parse_next(input)?;
        let v = Self { types, word };
        Ok((input, v))
    }
}

fn word(input: &str) -> IResult<&str, String> {
    winnow::bytes::take_till1(|item: char| item.is_ascii_whitespace())
        .map(|s: &str| s.to_owned().replace('_', " "))
        .parse_next(input)
}

#[cfg(test)]
mod test_variant {
    use super::*;

    #[test]
    fn test_valid() {
        // Having nothing after `A` causes an incomplete parse. Shouldn't be a problem for my use
        // cases.
        let (input, actual) = Variant::parse("A Cv: acknowledgment ").unwrap();
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
        let (input, actual) =
            Variant::parse("A Cv: acknowledgment's / Av B C: acknowledgement's").unwrap();
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
        let (input, actual) = Variant::parse("_: air_gun\n").unwrap();
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
    pub fn parse(input: &str) -> IResult<&str, Type> {
        let (input, category) = Category::parse(input)?;
        let (input, tag) = winnow::combinator::opt(Tag::parse).parse_next(input)?;
        let (input, num) = winnow::combinator::opt(winnow::character::digit1).parse_next(input)?;
        let num = num.map(|s| s.parse().expect("parser ensured its a number"));
        let t = Type { category, tag, num };
        Ok((input, t))
    }
}

#[cfg(test)]
mod test_type {
    use super::*;

    #[test]
    fn test_valid() {
        // Having nothing after `A` causes an incomplete parse. Shouldn't be a problem for my use
        // cases.
        let (input, actual) = Type::parse("A ").unwrap();
        assert_eq!(input, " ");
        assert_eq!(actual.category, Category::American);
        assert_eq!(actual.tag, None);
        assert_eq!(actual.num, None);

        let (input, actual) = Type::parse("Bv ").unwrap();
        assert_eq!(input, " ");
        assert_eq!(actual.category, Category::BritishIse);
        assert_eq!(actual.tag, Some(Tag::Variant));
        assert_eq!(actual.num, None);
    }

    #[test]
    fn test_extra() {
        let (input, actual) = Type::parse("Z foobar").unwrap();
        assert_eq!(input, " foobar");
        assert_eq!(actual.category, Category::BritishIze);
        assert_eq!(actual.tag, None);
        assert_eq!(actual.num, None);

        let (input, actual) = Type::parse("C- foobar").unwrap();
        assert_eq!(input, " foobar");
        assert_eq!(actual.category, Category::Canadian);
        assert_eq!(actual.tag, Some(Tag::Possible));
        assert_eq!(actual.num, None);
    }

    #[test]
    fn test_num() {
        let (input, actual) = Type::parse("Av1 ").unwrap();
        assert_eq!(input, " ");
        assert_eq!(actual.category, Category::American);
        assert_eq!(actual.tag, Some(Tag::Variant));
        assert_eq!(actual.num, Some(1));
    }
}

impl Category {
    pub fn parse(input: &str) -> IResult<&str, Category> {
        let symbols = winnow::bytes::one_of("ABZCD_");
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
    }
}

#[cfg(test)]
mod test_category {
    use super::*;

    #[test]
    fn test_valid() {
        let (input, actual) = Category::parse("A").unwrap();
        assert_eq!(input, "");
        assert_eq!(actual, Category::American);
    }

    #[test]
    fn test_extra() {
        let (input, actual) = Category::parse("_ foobar").unwrap();
        assert_eq!(input, " foobar");
        assert_eq!(actual, Category::Other);
    }
}

impl Tag {
    pub fn parse(input: &str) -> IResult<&str, Tag> {
        let symbols = winnow::bytes::one_of(".vV-x");
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
    }
}

#[cfg(test)]
mod test_tag {
    use super::*;

    #[test]
    fn test_valid() {
        let (input, actual) = Tag::parse(".").unwrap();
        assert_eq!(input, "");
        assert_eq!(actual, Tag::Eq);
    }

    #[test]
    fn test_extra() {
        let (input, actual) = Tag::parse("x foobar").unwrap();
        assert_eq!(input, " foobar");
        assert_eq!(actual, Tag::Improper);
    }
}

impl Pos {
    pub fn parse(input: &str) -> IResult<&str, Pos> {
        use winnow::bytes::tag;
        let noun = tag("<N>");
        let verb = tag("<V>");
        let adjective = tag("<Adj>");
        let adverb = tag("<Adv>");
        winnow::branch::alt((
            noun.value(Pos::Noun),
            verb.value(Pos::Verb),
            adjective.value(Pos::Adjective),
            adverb.value(Pos::Adverb),
        ))
        .parse_next(input)
    }
}

#[cfg(test)]
mod test_pos {
    use super::*;

    #[test]
    fn test_valid() {
        let (input, actual) = Pos::parse("<N>").unwrap();
        assert_eq!(input, "");
        assert_eq!(actual, Pos::Noun);
    }

    #[test]
    fn test_extra() {
        let (input, actual) = Pos::parse("<Adj> foobar").unwrap();
        assert_eq!(input, " foobar");
        assert_eq!(actual, Pos::Adjective);
    }
}
