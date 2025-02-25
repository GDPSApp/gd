use std::{borrow::Cow, error::Error, mem::take};

use serde::{ser::Error as _, Deserialize, Serialize, Serializer};

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(untagged)]
pub enum Thunk<'t, P: Processor> {
    Unprocessed(Cow<'t, str>),
    Processed(P::Output<'t>),
}

impl<'t, P: Processor> Default for Thunk<'t, P> {
    fn default() -> Self {
        Self::Unprocessed(Cow::Owned(String::new()))
    }
}

pub use Thunk::{Processed, Unprocessed};

impl<'t, P: Processor> Thunk<'t, P> {
    pub const fn is_unprocessed(&self) -> bool {
        matches!(self, Unprocessed(_))
    }

    pub const fn is_processed(&self) -> bool {
        matches!(self, Processed(_))
    }

    pub fn process(&mut self) -> Result<&mut P::Output<'t>, P::ProcessError> {
        if let Unprocessed(unprocessed) = self {
            let processed = P::process(take(unprocessed))?;

            *self = Processed(processed);
        };

        match self {
            Processed(processed) => Ok(processed),
            Unprocessed(_) => unreachable!(),
        }
    }

    pub fn unprocess(&mut self) {}

    pub fn to_unprocessed(&self) -> Result<Cow<'_, str>, P::UnprocessError> {
        match self {
            Unprocessed(unprocessed) => Ok(Cow::Borrowed(unprocessed)),
            Processed(processed) => P::unprocess(processed),
        }
    }

    pub fn into_processed(self) -> Result<P::Output<'t>, P::ProcessError> {
        match self {
            Unprocessed(unprocessed) => P::process(unprocessed),
            Processed(processed) => Ok(processed),
        }
    }
}

impl<'t, P: Processor> Serialize for Thunk<'t, P>
where
    for<'a> P::Output<'a>: Serialize,
{
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Unprocessed(unprocessed) => P::process(Cow::Borrowed(unprocessed))
                .map_err(S::Error::custom)?
                .serialize(serializer),
            Processed(processed) => processed.serialize(serializer),
        }
    }
}

pub trait Processor {
    type ProcessError: Error;
    type UnprocessError: Error;

    type Output<'a>;

    fn process(unprocessed: Cow<'_, str>) -> Result<Self::Output<'_>, Self::ProcessError>;
    fn unprocess<'p>(processed: &'p Self::Output<'_>)
        -> Result<Cow<'p, str>, Self::UnprocessError>;
}
