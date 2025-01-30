use winnow::ascii::space1;
use winnow::combinator::alt;
use winnow::combinator::cut_err;
use winnow::combinator::delimited;
use winnow::combinator::opt;
use winnow::combinator::preceded;
use winnow::combinator::terminated;
use winnow::combinator::trace;
use winnow::prelude::*;
use winnow::token::one_of;

use crate::{Category, Cluster, Entry, Pos, Tag, Type, Variant};

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClusterIter<'i> {
    input: &'i str,
}

impl<'i> ClusterIter<'i> {
    pub fn new(input: &'i str) -> Self {
        Self { input }
    }
}

impl Iterator for ClusterIter<'_> {
    type Item = Cluster;

    fn next(&mut self) -> Option<Cluster> {
        self.input = self.input.trim_start();
        Cluster::parse_.parse_next(&mut self.input).ok()
    }
}

#[cfg(test)]
mod test_cluster_iter {
    use super::*;

    use snapbox::assert_data_eq;
    use snapbox::str;
    use snapbox::ToDebug;

    #[test]
    fn test_single() {
        let actual = ClusterIter::new(
            "# acknowledgment <verified> (level 35)
A Cv: acknowledgment / Av B C: acknowledgement
A Cv: acknowledgments / Av B C: acknowledgements
A Cv: acknowledgment's / Av B C: acknowledgement's

",
        );
        assert_data_eq!(
            actual.collect::<Vec<_>>().to_debug(),
            str![[r#"
[
    Cluster {
        header: "acknowledgment ",
        verified: true,
        level: 35,
        entries: [
            Entry {
                variants: [
                    Variant {
                        types: [
                            Type {
                                category: American,
                                tag: None,
                                num: None,
                            },
                            Type {
                                category: Canadian,
                                tag: Some(
                                    Variant,
                                ),
                                num: None,
                            },
                        ],
                        word: "acknowledgment",
                    },
                    Variant {
                        types: [
                            Type {
                                category: American,
                                tag: Some(
                                    Variant,
                                ),
                                num: None,
                            },
                            Type {
                                category: BritishIse,
                                tag: None,
                                num: None,
                            },
                            Type {
                                category: Canadian,
                                tag: None,
                                num: None,
                            },
                        ],
                        word: "acknowledgement",
                    },
                ],
                pos: None,
                archaic: false,
                description: None,
                note: None,
                comment: None,
            },
            Entry {
                variants: [
                    Variant {
                        types: [
                            Type {
                                category: American,
                                tag: None,
                                num: None,
                            },
                            Type {
                                category: Canadian,
                                tag: Some(
                                    Variant,
                                ),
                                num: None,
                            },
                        ],
                        word: "acknowledgments",
                    },
                    Variant {
                        types: [
                            Type {
                                category: American,
                                tag: Some(
                                    Variant,
                                ),
                                num: None,
                            },
                            Type {
                                category: BritishIse,
                                tag: None,
                                num: None,
                            },
                            Type {
                                category: Canadian,
                                tag: None,
                                num: None,
                            },
                        ],
                        word: "acknowledgements",
                    },
                ],
                pos: None,
                archaic: false,
                description: None,
                note: None,
                comment: None,
            },
            Entry {
                variants: [
                    Variant {
                        types: [
                            Type {
                                category: American,
                                tag: None,
                                num: None,
                            },
                            Type {
                                category: Canadian,
                                tag: Some(
                                    Variant,
                                ),
                                num: None,
                            },
                        ],
                        word: "acknowledgment's",
                    },
                    Variant {
                        types: [
                            Type {
                                category: American,
                                tag: Some(
                                    Variant,
                                ),
                                num: None,
                            },
                            Type {
                                category: BritishIse,
                                tag: None,
                                num: None,
                            },
                            Type {
                                category: Canadian,
                                tag: None,
                                num: None,
                            },
                        ],
                        word: "acknowledgement's",
                    },
                ],
                pos: None,
                archaic: false,
                description: None,
                note: None,
                comment: None,
            },
        ],
        notes: [],
    },
]

"#]]
        );
    }

    #[test]
    fn test_multiple() {
        let actual = ClusterIter::new(
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
        assert_data_eq!(
            actual.collect::<Vec<_>>().to_debug(),
            str![[r#"
[
    Cluster {
        header: "acknowledgment ",
        verified: true,
        level: 35,
        entries: [
            Entry {
                variants: [
                    Variant {
                        types: [
                            Type {
                                category: American,
                                tag: None,
                                num: None,
                            },
                            Type {
                                category: Canadian,
                                tag: Some(
                                    Variant,
                                ),
                                num: None,
                            },
                        ],
                        word: "acknowledgment",
                    },
                    Variant {
                        types: [
                            Type {
                                category: American,
                                tag: Some(
                                    Variant,
                                ),
                                num: None,
                            },
                            Type {
                                category: BritishIse,
                                tag: None,
                                num: None,
                            },
                            Type {
                                category: Canadian,
                                tag: None,
                                num: None,
                            },
                        ],
                        word: "acknowledgement",
                    },
                ],
                pos: None,
                archaic: false,
                description: None,
                note: None,
                comment: None,
            },
            Entry {
                variants: [
                    Variant {
                        types: [
                            Type {
                                category: American,
                                tag: None,
                                num: None,
                            },
                            Type {
                                category: Canadian,
                                tag: Some(
                                    Variant,
                                ),
                                num: None,
                            },
                        ],
                        word: "acknowledgments",
                    },
                    Variant {
                        types: [
                            Type {
                                category: American,
                                tag: Some(
                                    Variant,
                                ),
                                num: None,
                            },
                            Type {
                                category: BritishIse,
                                tag: None,
                                num: None,
                            },
                            Type {
                                category: Canadian,
                                tag: None,
                                num: None,
                            },
                        ],
                        word: "acknowledgements",
                    },
                ],
                pos: None,
                archaic: false,
                description: None,
                note: None,
                comment: None,
            },
            Entry {
                variants: [
                    Variant {
                        types: [
                            Type {
                                category: American,
                                tag: None,
                                num: None,
                            },
                            Type {
                                category: Canadian,
                                tag: Some(
                                    Variant,
                                ),
                                num: None,
                            },
                        ],
                        word: "acknowledgment's",
                    },
                    Variant {
                        types: [
                            Type {
                                category: American,
                                tag: Some(
                                    Variant,
                                ),
                                num: None,
                            },
                            Type {
                                category: BritishIse,
                                tag: None,
                                num: None,
                            },
                            Type {
                                category: Canadian,
                                tag: None,
                                num: None,
                            },
                        ],
                        word: "acknowledgement's",
                    },
                ],
                pos: None,
                archaic: false,
                description: None,
                note: None,
                comment: None,
            },
        ],
        notes: [],
    },
    Cluster {
        header: "acknowledgment ",
        verified: true,
        level: 35,
        entries: [
            Entry {
                variants: [
                    Variant {
                        types: [
                            Type {
                                category: American,
                                tag: None,
                                num: None,
                            },
                            Type {
                                category: Canadian,
                                tag: Some(
                                    Variant,
                                ),
                                num: None,
                            },
                        ],
                        word: "acknowledgment",
                    },
                    Variant {
                        types: [
                            Type {
                                category: American,
                                tag: Some(
                                    Variant,
                                ),
                                num: None,
                            },
                            Type {
                                category: BritishIse,
                                tag: None,
                                num: None,
                            },
                            Type {
                                category: Canadian,
                                tag: None,
                                num: None,
                            },
                        ],
                        word: "acknowledgement",
                    },
                ],
                pos: None,
                archaic: false,
                description: None,
                note: None,
                comment: None,
            },
            Entry {
                variants: [
                    Variant {
                        types: [
                            Type {
                                category: American,
                                tag: None,
                                num: None,
                            },
                            Type {
                                category: Canadian,
                                tag: Some(
                                    Variant,
                                ),
                                num: None,
                            },
                        ],
                        word: "acknowledgments",
                    },
                    Variant {
                        types: [
                            Type {
                                category: American,
                                tag: Some(
                                    Variant,
                                ),
                                num: None,
                            },
                            Type {
                                category: BritishIse,
                                tag: None,
                                num: None,
                            },
                            Type {
                                category: Canadian,
                                tag: None,
                                num: None,
                            },
                        ],
                        word: "acknowledgements",
                    },
                ],
                pos: None,
                archaic: false,
                description: None,
                note: None,
                comment: None,
            },
            Entry {
                variants: [
                    Variant {
                        types: [
                            Type {
                                category: American,
                                tag: None,
                                num: None,
                            },
                            Type {
                                category: Canadian,
                                tag: Some(
                                    Variant,
                                ),
                                num: None,
                            },
                        ],
                        word: "acknowledgment's",
                    },
                    Variant {
                        types: [
                            Type {
                                category: American,
                                tag: Some(
                                    Variant,
                                ),
                                num: None,
                            },
                            Type {
                                category: BritishIse,
                                tag: None,
                                num: None,
                            },
                            Type {
                                category: Canadian,
                                tag: None,
                                num: None,
                            },
                        ],
                        word: "acknowledgement's",
                    },
                ],
                pos: None,
                archaic: false,
                description: None,
                note: None,
                comment: None,
            },
        ],
        notes: [],
    },
]

"#]]
        );
    }
}

impl Cluster {
    pub fn parse(input: &str) -> Result<Self, ParseError> {
        Self::parse_.parse(input).map_err(|_err| ParseError)
    }

    fn parse_(input: &mut &str) -> ModalResult<Self, ()> {
        trace("cluster", move |input: &mut &str| {
            let header = (
                "#",
                winnow::ascii::space0,
                winnow::token::take_till(1.., ('\r', '\n', '<', '(')),
                winnow::ascii::space0,
                opt(("<verified>", winnow::ascii::space0)),
                delimited("(level ", winnow::ascii::digit1, ')').parse_to::<usize>(),
                winnow::ascii::space0,
                winnow::ascii::line_ending,
            );
            let note = preceded(
                ("##", winnow::ascii::space0),
                terminated(winnow::ascii::till_line_ending, winnow::ascii::line_ending),
            );
            let mut cluster = (
                header,
                winnow::combinator::repeat(
                    1..,
                    terminated(Entry::parse_, winnow::ascii::line_ending),
                ),
                winnow::combinator::repeat(0.., note),
            );
            let (header, entries, notes): (_, _, Vec<_>) = cluster.parse_next(input)?;

            let verified = header.4.is_some();
            let level = header.5;
            let header = header.2.to_owned();
            let notes = notes.into_iter().map(|s| s.to_owned()).collect();
            let c = Self {
                header,
                verified,
                level,
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

    use snapbox::assert_data_eq;
    use snapbox::str;
    use snapbox::ToDebug;

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
        assert_data_eq!(
            input,
            str![[r#"


"#]]
        );
        assert_data_eq!(
            actual.to_debug(),
            str![[r#"
Cluster {
    header: "acknowledgment ",
    verified: true,
    level: 35,
    entries: [
        Entry {
            variants: [
                Variant {
                    types: [
                        Type {
                            category: American,
                            tag: None,
                            num: None,
                        },
                        Type {
                            category: Canadian,
                            tag: Some(
                                Variant,
                            ),
                            num: None,
                        },
                    ],
                    word: "acknowledgment",
                },
                Variant {
                    types: [
                        Type {
                            category: American,
                            tag: Some(
                                Variant,
                            ),
                            num: None,
                        },
                        Type {
                            category: BritishIse,
                            tag: None,
                            num: None,
                        },
                        Type {
                            category: Canadian,
                            tag: None,
                            num: None,
                        },
                    ],
                    word: "acknowledgement",
                },
            ],
            pos: None,
            archaic: false,
            description: None,
            note: None,
            comment: None,
        },
        Entry {
            variants: [
                Variant {
                    types: [
                        Type {
                            category: American,
                            tag: None,
                            num: None,
                        },
                        Type {
                            category: Canadian,
                            tag: Some(
                                Variant,
                            ),
                            num: None,
                        },
                    ],
                    word: "acknowledgments",
                },
                Variant {
                    types: [
                        Type {
                            category: American,
                            tag: Some(
                                Variant,
                            ),
                            num: None,
                        },
                        Type {
                            category: BritishIse,
                            tag: None,
                            num: None,
                        },
                        Type {
                            category: Canadian,
                            tag: None,
                            num: None,
                        },
                    ],
                    word: "acknowledgements",
                },
            ],
            pos: None,
            archaic: false,
            description: None,
            note: None,
            comment: None,
        },
        Entry {
            variants: [
                Variant {
                    types: [
                        Type {
                            category: American,
                            tag: None,
                            num: None,
                        },
                        Type {
                            category: Canadian,
                            tag: Some(
                                Variant,
                            ),
                            num: None,
                        },
                    ],
                    word: "acknowledgment's",
                },
                Variant {
                    types: [
                        Type {
                            category: American,
                            tag: Some(
                                Variant,
                            ),
                            num: None,
                        },
                        Type {
                            category: BritishIse,
                            tag: None,
                            num: None,
                        },
                        Type {
                            category: Canadian,
                            tag: None,
                            num: None,
                        },
                    ],
                    word: "acknowledgement's",
                },
            ],
            pos: None,
            archaic: false,
            description: None,
            note: None,
            comment: None,
        },
    ],
    notes: [],
}

"#]]
        );
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
        assert_data_eq!(
            input,
            str![[r#"


"#]]
        );
        assert_data_eq!(
            actual.to_debug(),
            str![[r#"
Cluster {
    header: "coloration ",
    verified: true,
    level: 50,
    entries: [
        Entry {
            variants: [
                Variant {
                    types: [
                        Type {
                            category: American,
                            tag: None,
                            num: None,
                        },
                        Type {
                            category: BritishIse,
                            tag: None,
                            num: None,
                        },
                        Type {
                            category: Canadian,
                            tag: None,
                            num: None,
                        },
                    ],
                    word: "coloration",
                },
                Variant {
                    types: [
                        Type {
                            category: BritishIse,
                            tag: Some(
                                Eq,
                            ),
                            num: None,
                        },
                        Type {
                            category: Canadian,
                            tag: Some(
                                Variant,
                            ),
                            num: None,
                        },
                    ],
                    word: "colouration",
                },
            ],
            pos: None,
            archaic: false,
            description: None,
            note: None,
            comment: None,
        },
        Entry {
            variants: [
                Variant {
                    types: [
                        Type {
                            category: American,
                            tag: None,
                            num: None,
                        },
                        Type {
                            category: BritishIse,
                            tag: None,
                            num: None,
                        },
                        Type {
                            category: Canadian,
                            tag: None,
                            num: None,
                        },
                    ],
                    word: "colorations",
                },
                Variant {
                    types: [
                        Type {
                            category: BritishIse,
                            tag: Some(
                                Eq,
                            ),
                            num: None,
                        },
                        Type {
                            category: Canadian,
                            tag: Some(
                                Variant,
                            ),
                            num: None,
                        },
                    ],
                    word: "colourations",
                },
            ],
            pos: None,
            archaic: false,
            description: None,
            note: None,
            comment: None,
        },
        Entry {
            variants: [
                Variant {
                    types: [
                        Type {
                            category: American,
                            tag: None,
                            num: None,
                        },
                        Type {
                            category: BritishIse,
                            tag: None,
                            num: None,
                        },
                        Type {
                            category: Canadian,
                            tag: None,
                            num: None,
                        },
                    ],
                    word: "coloration's",
                },
                Variant {
                    types: [
                        Type {
                            category: BritishIse,
                            tag: Some(
                                Eq,
                            ),
                            num: None,
                        },
                        Type {
                            category: Canadian,
                            tag: Some(
                                Variant,
                            ),
                            num: None,
                        },
                    ],
                    word: "colouration's",
                },
            ],
            pos: None,
            archaic: false,
            description: None,
            note: None,
            comment: None,
        },
    ],
    notes: [
        "OED has coloration as the preferred spelling and discolouration as a",
        "variant for British Engl or some reason",
    ],
}

"#]]
        );
    }
}

impl Entry {
    pub fn parse(input: &str) -> Result<Self, ParseError> {
        Self::parse_.parse(input).map_err(|_err| ParseError)
    }

    fn parse_(input: &mut &str) -> ModalResult<Self, ()> {
        trace("entry", move |input: &mut &str| {
            let var_sep = (winnow::ascii::space0, '/', winnow::ascii::space0);
            let variants =
                winnow::combinator::separated(1.., Variant::parse_, var_sep).parse_next(input)?;

            let mut e = Self::parse_description.parse_next(input)?;

            let comment_sep = (winnow::ascii::space0, '#');
            let comment =
                opt((comment_sep, space1, winnow::ascii::till_line_ending)).parse_next(input)?;

            let _ = winnow::ascii::space0.parse_next(input)?;

            e.variants = variants;
            e.comment = comment.map(|c| c.2.to_owned());
            Ok(e)
        })
        .parse_next(input)
    }

    fn parse_description(input: &mut &str) -> ModalResult<Self, ()> {
        trace("description", move |input: &mut &str| {
            let mut entry = Self {
                variants: Vec::new(),
                pos: None,
                archaic: false,
                description: None,
                note: None,
                comment: None,
            };

            if opt((winnow::ascii::space0, '|'))
                .parse_next(input)?
                .is_some()
            {
                let _ = opt((space1, "<abbr>")).parse_next(input)?;
                let _ = opt((space1, "<pl>")).parse_next(input)?;
                entry.pos = opt(delimited((space1, '<'), cut_err(Pos::parse_), cut_err('>')))
                    .parse_next(input)?;
                entry.archaic = opt(preceded(space1, archaic)).parse_next(input)?.is_some();
                entry.note = opt(preceded(space1, note)).parse_next(input)?;
                entry.description = opt(preceded(space1, description)).parse_next(input)?;

                if opt((winnow::ascii::space0, '|'))
                    .parse_next(input)?
                    .is_some()
                {
                    entry.note = opt(preceded(space1, note)).parse_next(input)?;
                }
            }
            Ok(entry)
        })
        .parse_next(input)
    }
}

fn note(input: &mut &str) -> ModalResult<String, ()> {
    let (_, _, note) = (NOTE_PREFIX, space1, description).parse_next(input)?;
    Ok(note)
}

const NOTE_PREFIX: &str = "--";

fn archaic(input: &mut &str) -> ModalResult<(), ()> {
    "(-)".void().parse_next(input)
}

fn description(input: &mut &str) -> ModalResult<String, ()> {
    let description = winnow::token::take_till(0.., ('\n', '\r', '#', '|')).parse_next(input)?;
    Ok(description.to_owned())
}

#[cfg(test)]
mod test_entry {
    #![allow(clippy::bool_assert_comparison)]
    use super::*;

    use snapbox::assert_data_eq;
    use snapbox::str;
    use snapbox::ToDebug;

    #[test]
    fn test_variant_only() {
        // Having nothing after `A` causes an incomplete parse. Shouldn't be a problem for my use
        // cases.
        let (input, actual) = Entry::parse_
            .parse_peek("A Cv: acknowledgment's / Av B C: acknowledgement's\n")
            .unwrap();
        assert_data_eq!(
            input,
            str![[r#"


"#]]
        );
        assert_data_eq!(
            actual.to_debug(),
            str![[r#"
Entry {
    variants: [
        Variant {
            types: [
                Type {
                    category: American,
                    tag: None,
                    num: None,
                },
                Type {
                    category: Canadian,
                    tag: Some(
                        Variant,
                    ),
                    num: None,
                },
            ],
            word: "acknowledgment's",
        },
        Variant {
            types: [
                Type {
                    category: American,
                    tag: Some(
                        Variant,
                    ),
                    num: None,
                },
                Type {
                    category: BritishIse,
                    tag: None,
                    num: None,
                },
                Type {
                    category: Canadian,
                    tag: None,
                    num: None,
                },
            ],
            word: "acknowledgement's",
        },
    ],
    pos: None,
    archaic: false,
    description: None,
    note: None,
    comment: None,
}

"#]]
        );
    }

    #[test]
    fn test_description() {
        // Having nothing after `A` causes an incomplete parse. Shouldn't be a problem for my use
        // cases.
        let (input, actual) = Entry::parse_
            .parse_peek("A C: prize / B: prise | otherwise\n")
            .unwrap();
        assert_data_eq!(
            input,
            str![[r#"


"#]]
        );
        assert_data_eq!(
            actual.to_debug(),
            str![[r#"
Entry {
    variants: [
        Variant {
            types: [
                Type {
                    category: American,
                    tag: None,
                    num: None,
                },
                Type {
                    category: Canadian,
                    tag: None,
                    num: None,
                },
            ],
            word: "prize",
        },
        Variant {
            types: [
                Type {
                    category: BritishIse,
                    tag: None,
                    num: None,
                },
            ],
            word: "prise",
        },
    ],
    pos: None,
    archaic: false,
    description: Some(
        "otherwise",
    ),
    note: None,
    comment: None,
}

"#]]
        );
    }

    #[test]
    fn test_pos() {
        // Having nothing after `A` causes an incomplete parse. Shouldn't be a problem for my use
        // cases.
        let (input, actual) = Entry::parse_
            .parse_peek("A B C: practice / AV Cv: practise | <N>\n")
            .unwrap();
        assert_data_eq!(
            input,
            str![[r#"


"#]]
        );
        assert_data_eq!(
            actual.to_debug(),
            str![[r#"
Entry {
    variants: [
        Variant {
            types: [
                Type {
                    category: American,
                    tag: None,
                    num: None,
                },
                Type {
                    category: BritishIse,
                    tag: None,
                    num: None,
                },
                Type {
                    category: Canadian,
                    tag: None,
                    num: None,
                },
            ],
            word: "practice",
        },
        Variant {
            types: [
                Type {
                    category: American,
                    tag: Some(
                        Seldom,
                    ),
                    num: None,
                },
                Type {
                    category: Canadian,
                    tag: Some(
                        Variant,
                    ),
                    num: None,
                },
            ],
            word: "practise",
        },
    ],
    pos: Some(
        Noun,
    ),
    archaic: false,
    description: None,
    note: None,
    comment: None,
}

"#]]
        );
    }

    #[test]
    fn test_pos_bad() {
        // Having nothing after `A` causes an incomplete parse. Shouldn't be a problem for my use
        // cases.
        let err = Entry::parse_
            .parse_peek("A B C: practice / AV Cv: practise | <Bad>\n")
            .unwrap_err();
        assert_data_eq!(err.to_string(), str!["Parsing Failure: ()"]);
    }

    #[test]
    fn test_plural() {
        // Having nothing after `A` causes an incomplete parse. Shouldn't be a problem for my use
        // cases.
        let (input, actual) = Entry::parse_.parse_peek("_ _-: dogies | <pl>\n").unwrap();
        assert_data_eq!(
            input,
            str![[r#"


"#]]
        );
        assert_data_eq!(
            actual.to_debug(),
            str![[r#"
Entry {
    variants: [
        Variant {
            types: [
                Type {
                    category: Other,
                    tag: None,
                    num: None,
                },
                Type {
                    category: Other,
                    tag: Some(
                        Possible,
                    ),
                    num: None,
                },
            ],
            word: "dogies",
        },
    ],
    pos: None,
    archaic: false,
    description: None,
    note: None,
    comment: None,
}

"#]]
        );
    }

    #[test]
    fn test_abbr() {
        // Having nothing after `A` causes an incomplete parse. Shouldn't be a problem for my use
        // cases.
        let (input, actual) = Entry::parse_.parse_peek("A B: ha | <abbr>\n").unwrap();
        assert_data_eq!(
            input,
            str![[r#"


"#]]
        );
        assert_data_eq!(
            actual.to_debug(),
            str![[r#"
Entry {
    variants: [
        Variant {
            types: [
                Type {
                    category: American,
                    tag: None,
                    num: None,
                },
                Type {
                    category: BritishIse,
                    tag: None,
                    num: None,
                },
            ],
            word: "ha",
        },
    ],
    pos: None,
    archaic: false,
    description: None,
    note: None,
    comment: None,
}

"#]]
        );
    }

    #[test]
    fn test_archaic() {
        // Having nothing after `A` causes an incomplete parse. Shouldn't be a problem for my use
        // cases.
        let (input, actual) = Entry::parse_
            .parse_peek("A: bark / Av B: barque | (-) ship\n")
            .unwrap();
        assert_data_eq!(
            input,
            str![[r#"


"#]]
        );
        assert_data_eq!(
            actual.to_debug(),
            str![[r#"
Entry {
    variants: [
        Variant {
            types: [
                Type {
                    category: American,
                    tag: None,
                    num: None,
                },
            ],
            word: "bark",
        },
        Variant {
            types: [
                Type {
                    category: American,
                    tag: Some(
                        Variant,
                    ),
                    num: None,
                },
                Type {
                    category: BritishIse,
                    tag: None,
                    num: None,
                },
            ],
            word: "barque",
        },
    ],
    pos: None,
    archaic: true,
    description: Some(
        "ship",
    ),
    note: None,
    comment: None,
}

"#]]
        );
    }

    #[test]
    fn test_note() {
        // Having nothing after `A` causes an incomplete parse. Shouldn't be a problem for my use
        // cases.
        let (input, actual) = Entry::parse_
            .parse_peek("_: cabbies | -- plural\n")
            .unwrap();
        assert_data_eq!(
            input,
            str![[r#"


"#]]
        );
        assert_data_eq!(
            actual.to_debug(),
            str![[r#"
Entry {
    variants: [
        Variant {
            types: [
                Type {
                    category: Other,
                    tag: None,
                    num: None,
                },
            ],
            word: "cabbies",
        },
    ],
    pos: None,
    archaic: false,
    description: None,
    note: Some(
        "plural",
    ),
    comment: None,
}

"#]]
        );
    }

    #[test]
    fn test_description_and_note() {
        // Having nothing after `A` causes an incomplete parse. Shouldn't be a problem for my use
        // cases.
        let (input, actual) = Entry::parse_
            .parse_peek("A B: wizz | as in \"gee whiz\" | -- Ox: informal, chiefly N. Amer.\n")
            .unwrap();
        assert_data_eq!(
            input,
            str![[r#"


"#]]
        );
        assert_data_eq!(
            actual.to_debug(),
            str![[r#"
Entry {
    variants: [
        Variant {
            types: [
                Type {
                    category: American,
                    tag: None,
                    num: None,
                },
                Type {
                    category: BritishIse,
                    tag: None,
                    num: None,
                },
            ],
            word: "wizz",
        },
    ],
    pos: None,
    archaic: false,
    description: Some(
        "as in /"gee whiz/" ",
    ),
    note: Some(
        "Ox: informal, chiefly N. Amer.",
    ),
    comment: None,
}

"#]]
        );
    }

    #[test]
    fn test_trailing_comment() {
        let (input, actual) = Entry::parse_.parse_peek(
            "A B: accursed / AV B-: accurst # ODE: archaic, M-W: 'or' but can find little evidence of use\n",
        )
        .unwrap();
        assert_data_eq!(
            input,
            str![[r#"


"#]]
        );
        assert_data_eq!(
            actual.to_debug(),
            str![[r#"
Entry {
    variants: [
        Variant {
            types: [
                Type {
                    category: American,
                    tag: None,
                    num: None,
                },
                Type {
                    category: BritishIse,
                    tag: None,
                    num: None,
                },
            ],
            word: "accursed",
        },
        Variant {
            types: [
                Type {
                    category: American,
                    tag: Some(
                        Seldom,
                    ),
                    num: None,
                },
                Type {
                    category: BritishIse,
                    tag: Some(
                        Possible,
                    ),
                    num: None,
                },
            ],
            word: "accurst",
        },
    ],
    pos: None,
    archaic: false,
    description: None,
    note: None,
    comment: Some(
        "ODE: archaic, M-W: 'or' but can find little evidence of use",
    ),
}

"#]]
        );
    }
}

impl Variant {
    pub fn parse(input: &str) -> Result<Self, ParseError> {
        Self::parse_.parse(input).map_err(|_err| ParseError)
    }

    fn parse_(input: &mut &str) -> ModalResult<Self, ()> {
        trace("variant", move |input: &mut &str| {
            let types = winnow::combinator::separated(1.., Type::parse_, space1);
            let columns =
                winnow::combinator::separated(0.., winnow::ascii::digit1, space1).map(|()| ());
            let sep = (":", winnow::ascii::space0);
            let ((types, _, _columns), word) = winnow::combinator::separated_pair(
                (types, winnow::ascii::space0, columns),
                sep,
                word,
            )
            .parse_next(input)?;
            let v = Self { types, word };
            Ok(v)
        })
        .parse_next(input)
    }
}

fn word(input: &mut &str) -> ModalResult<String, ()> {
    trace("word", move |input: &mut &str| {
        winnow::token::take_till(1.., |item: char| item.is_ascii_whitespace())
            .map(|s: &str| s.to_owned().replace('_', " "))
            .parse_next(input)
    })
    .parse_next(input)
}

#[cfg(test)]
mod test_variant {
    use super::*;

    use snapbox::assert_data_eq;
    use snapbox::str;
    use snapbox::ToDebug;

    #[test]
    fn test_valid() {
        // Having nothing after `A` causes an incomplete parse. Shouldn't be a problem for my use
        // cases.
        let (input, actual) = Variant::parse_.parse_peek("A Cv: acknowledgment ").unwrap();
        assert_data_eq!(input, str![" "]);
        assert_data_eq!(
            actual.to_debug(),
            str![[r#"
Variant {
    types: [
        Type {
            category: American,
            tag: None,
            num: None,
        },
        Type {
            category: Canadian,
            tag: Some(
                Variant,
            ),
            num: None,
        },
    ],
    word: "acknowledgment",
}

"#]]
        );
    }

    #[test]
    fn test_extra() {
        let (input, actual) = Variant::parse_
            .parse_peek("A Cv: acknowledgment's / Av B C: acknowledgement's")
            .unwrap();
        assert_data_eq!(input, str![" / Av B C: acknowledgement's"]);
        assert_data_eq!(
            actual.to_debug(),
            str![[r#"
Variant {
    types: [
        Type {
            category: American,
            tag: None,
            num: None,
        },
        Type {
            category: Canadian,
            tag: Some(
                Variant,
            ),
            num: None,
        },
    ],
    word: "acknowledgment's",
}

"#]]
        );
    }

    #[test]
    fn test_underscore() {
        let (input, actual) = Variant::parse_.parse_peek("_: air_gun\n").unwrap();
        assert_data_eq!(
            input,
            str![[r#"


"#]]
        );
        assert_data_eq!(
            actual.to_debug(),
            str![[r#"
Variant {
    types: [
        Type {
            category: Other,
            tag: None,
            num: None,
        },
    ],
    word: "air gun",
}

"#]]
        );
    }

    #[test]
    fn test_columns() {
        // Having nothing after `A` causes an incomplete parse. Shouldn't be a problem for my use
        // cases.
        let (input, actual) = Variant::parse_.parse_peek("A B 1 2: aeries").unwrap();
        assert_data_eq!(input, str![""]);
        assert_data_eq!(
            actual.to_debug(),
            str![[r#"
Variant {
    types: [
        Type {
            category: American,
            tag: None,
            num: None,
        },
        Type {
            category: BritishIse,
            tag: None,
            num: None,
        },
    ],
    word: "aeries",
}

"#]]
        );
    }
}

impl Type {
    pub fn parse(input: &str) -> Result<Self, ParseError> {
        Self::parse_.parse(input).map_err(|_err| ParseError)
    }

    fn parse_(input: &mut &str) -> ModalResult<Type, ()> {
        trace("type", move |input: &mut &str| {
            let category = Category::parse_(input)?;
            let tag = opt(Tag::parse_).parse_next(input)?;
            let num = opt(winnow::ascii::digit1).parse_next(input)?;
            let num = num.map(|s| s.parse().expect("parser ensured it's a number"));
            let t = Type { category, tag, num };
            Ok(t)
        })
        .parse_next(input)
    }
}

#[cfg(test)]
mod test_type {
    use super::*;

    use snapbox::assert_data_eq;
    use snapbox::str;
    use snapbox::ToDebug;

    #[test]
    fn test_valid() {
        // Having nothing after `A` causes an incomplete parse. Shouldn't be a problem for my use
        // cases.
        let (input, actual) = Type::parse_.parse_peek("A ").unwrap();
        assert_data_eq!(input, str![" "]);
        assert_data_eq!(
            actual.to_debug(),
            str![[r#"
Type {
    category: American,
    tag: None,
    num: None,
}

"#]]
        );

        let (input, actual) = Type::parse_.parse_peek("Bv ").unwrap();
        assert_data_eq!(input, str![" "]);
        assert_data_eq!(
            actual.to_debug(),
            str![[r#"
Type {
    category: BritishIse,
    tag: Some(
        Variant,
    ),
    num: None,
}

"#]]
        );
    }

    #[test]
    fn test_extra() {
        let (input, actual) = Type::parse_.parse_peek("Z foobar").unwrap();
        assert_data_eq!(input, str![" foobar"]);
        assert_data_eq!(
            actual.to_debug(),
            str![[r#"
Type {
    category: BritishIze,
    tag: None,
    num: None,
}

"#]]
        );

        let (input, actual) = Type::parse_.parse_peek("C- foobar").unwrap();
        assert_data_eq!(input, str![" foobar"]);
        assert_data_eq!(
            actual.to_debug(),
            str![[r#"
Type {
    category: Canadian,
    tag: Some(
        Possible,
    ),
    num: None,
}

"#]]
        );
    }

    #[test]
    fn test_num() {
        let (input, actual) = Type::parse_.parse_peek("Av1 ").unwrap();
        assert_data_eq!(input, str![" "]);
        assert_data_eq!(
            actual.to_debug(),
            str![[r#"
Type {
    category: American,
    tag: Some(
        Variant,
    ),
    num: Some(
        1,
    ),
}

"#]]
        );
    }
}

impl Category {
    pub fn parse(input: &str) -> Result<Self, ParseError> {
        Self::parse_.parse(input).map_err(|_err| ParseError)
    }

    fn parse_(input: &mut &str) -> ModalResult<Self, ()> {
        trace("category", move |input: &mut &str| {
            let symbols = one_of(['A', 'B', 'Z', 'C', 'D', '_']);
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

    use snapbox::assert_data_eq;
    use snapbox::str;
    use snapbox::ToDebug;

    #[test]
    fn test_valid() {
        let (input, actual) = Category::parse_.parse_peek("A").unwrap();
        assert_data_eq!(input, str![]);
        assert_data_eq!(
            actual.to_debug(),
            str![[r#"
American

"#]]
        );
    }

    #[test]
    fn test_extra() {
        let (input, actual) = Category::parse_.parse_peek("_ foobar").unwrap();
        assert_data_eq!(input, str![" foobar"]);
        assert_data_eq!(
            actual.to_debug(),
            str![[r#"
Other

"#]]
        );
    }
}

impl Tag {
    pub fn parse(input: &str) -> Result<Self, ParseError> {
        Self::parse_.parse(input).map_err(|_err| ParseError)
    }

    fn parse_(input: &mut &str) -> ModalResult<Self, ()> {
        trace("tag", move |input: &mut &str| {
            let symbols = one_of(['.', 'v', 'V', '-', 'x']);
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

    use snapbox::assert_data_eq;
    use snapbox::str;
    use snapbox::ToDebug;

    #[test]
    fn test_valid() {
        let (input, actual) = Tag::parse_.parse_peek(".").unwrap();
        assert_data_eq!(input, str![]);
        assert_data_eq!(
            actual.to_debug(),
            str![[r#"
Eq

"#]]
        );
    }

    #[test]
    fn test_extra() {
        let (input, actual) = Tag::parse_.parse_peek("x foobar").unwrap();
        assert_data_eq!(input, str![" foobar"]);
        assert_data_eq!(
            actual.to_debug(),
            str![[r#"
Improper

"#]]
        );
    }
}

impl Pos {
    pub fn parse(input: &str) -> Result<Self, ParseError> {
        Self::parse_.parse(input).map_err(|_err| ParseError)
    }

    fn parse_(input: &mut &str) -> ModalResult<Self, ()> {
        trace("pos", move |input: &mut &str| {
            alt((
                "N".value(Pos::Noun),
                "V".value(Pos::Verb),
                "Adj".value(Pos::Adjective),
                "Adv".value(Pos::Adverb),
                "A".value(Pos::AdjectiveOrAdverb),
                "Inj".value(Pos::Interjection),
                "Prep".value(Pos::Preposition),
            ))
            .parse_next(input)
        })
        .parse_next(input)
    }
}

#[cfg(test)]
mod test_pos {
    use super::*;

    use snapbox::assert_data_eq;
    use snapbox::str;
    use snapbox::ToDebug;

    #[test]
    fn test_valid() {
        let (input, actual) = Pos::parse_.parse_peek("N>").unwrap();
        assert_data_eq!(input, str![">"]);
        assert_data_eq!(
            actual.to_debug(),
            str![[r#"
Noun

"#]]
        );
    }

    #[test]
    fn test_extra() {
        let (input, actual) = Pos::parse_.parse_peek("Adj> foobar").unwrap();
        assert_data_eq!(input, str!["> foobar"]);
        assert_data_eq!(
            actual.to_debug(),
            str![[r#"
Adjective

"#]]
        );
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
