use std::borrow::Cow;

/// Look up the validity of a term.
pub trait Dictionary: Send + Sync {
    /// Look up the validity of an Identifier.
    ///
    /// `None` if the status is unknown.
    fn correct_ident<'s>(&'s self, ident: crate::tokens::Identifier<'_>) -> Option<Status<'s>>;

    /// Look up the validity of a Word.
    ///
    /// `None` if the status is unknown.
    fn correct_word<'s>(&'s self, word: crate::tokens::Word<'_>) -> Option<Status<'s>>;
}

/// Validity of a term in a Dictionary.
#[derive(Clone, PartialEq, Eq, Debug, serde::Serialize)]
#[serde(rename_all = "snake_case")]
#[serde(untagged)]
pub enum Status<'c> {
    Valid,
    Invalid,
    Corrections(Vec<Cow<'c, str>>),
}

impl<'c> Status<'c> {
    #[inline]
    pub fn is_invalid(&self) -> bool {
        matches!(self, Status::Invalid)
    }
    #[inline]
    pub fn is_valid(&self) -> bool {
        matches!(self, Status::Valid)
    }
    #[inline]
    pub fn is_correction(&self) -> bool {
        matches!(self, Status::Corrections(_))
    }

    #[inline]
    pub fn corrections_mut(&mut self) -> impl Iterator<Item = &mut Cow<'c, str>> {
        match self {
            Status::Corrections(corrections) => itertools::Either::Left(corrections.iter_mut()),
            _ => itertools::Either::Right([].iter_mut()),
        }
    }

    pub fn into_owned(self) -> Status<'static> {
        match self {
            Status::Valid => Status::Valid,
            Status::Invalid => Status::Invalid,
            Status::Corrections(corrections) => {
                let corrections = corrections
                    .into_iter()
                    .map(|c| Cow::Owned(c.into_owned()))
                    .collect();
                Status::Corrections(corrections)
            }
        }
    }

    pub fn borrow(&self) -> Status<'_> {
        match self {
            Status::Corrections(corrections) => {
                let corrections = corrections
                    .iter()
                    .map(|c| Cow::Borrowed(c.as_ref()))
                    .collect();
                Status::Corrections(corrections)
            }
            _ => self.clone(),
        }
    }
}
