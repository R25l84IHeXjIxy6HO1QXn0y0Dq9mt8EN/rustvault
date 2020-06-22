use std::fmt;
use std::error;

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct VaultError<'a> {
    #[serde(borrow)]
    error: ErrorContainer<'a>
}

#[derive(Debug, Deserialize)]
struct ErrorContainer<'a> {
    code: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    innererror: Box<Option<ErrorContainer<'a>>>,
    message: &'a str
}

impl fmt::Display for VaultError<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "VaultError({}): {}", self.error.code, self.error.message)
    }
}

impl error::Error for VaultError<'_> { }
