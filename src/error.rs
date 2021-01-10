use std::fmt::{Debug, Display, Error, Formatter};
use std::io;

pub enum HeaviError {
    IOError(io::Error),
    RegexError(regex::Error),
}
impl Display for HeaviError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        match &self {
            HeaviError::IOError(x) => write!(f, "{}", x),
            HeaviError::RegexError(x) => write!(f, "{}", x),
        }
    }
}
impl Debug for HeaviError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        match &self {
            HeaviError::IOError(x) => write!(f, "IOError({})", x),
            HeaviError::RegexError(x) => write!(f, "RegexError({})", x),
        }
    }
}
impl From<io::Error> for HeaviError {
    fn from(err: io::Error) -> HeaviError {
        HeaviError::IOError(err)
    }
}
impl From<regex::Error> for HeaviError {
    fn from(err: regex::Error) -> HeaviError {
        HeaviError::RegexError(err)
    }
}
