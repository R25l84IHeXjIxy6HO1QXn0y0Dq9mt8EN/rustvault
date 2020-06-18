/*
mod certificate;
mod secret;
mod key;
mod storage;
*/

mod constants;
mod util;

pub mod secret;

use std::fmt;

pub type JSONResult<T> = Result<T, Box<dyn std::error::Error>>;

#[derive(Debug)]
pub struct Vault<'a> {
    name: &'a str,
    token: &'a str
}

impl fmt::Display for Vault<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "https://{}.vault.azure.net", self.name)
    }
}
