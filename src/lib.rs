/*
mod certificate;
mod secret;
mod key;
mod storage;
*/

mod constants;
mod error;
mod secret;
mod util;

use std::fmt;

use hyper::{Request, Uri};
use hyper::http::request::Builder;
use hyper::http::uri::InvalidUri;
use serde::{Deserialize, Serialize};

pub type BoxedResult<T> = Result<T, Box<dyn std::error::Error>>;


#[derive(Debug, Deserialize, Serialize)]
pub enum DeletionRecoveryLevel {
    Purgeable,
    Recoverable,
    #[serde(rename = "Recoverable+ProtectedSubscription")]
    RecoverableAndProtectedSubscription,
    #[serde(rename = "Recoverable+Purgeable")]
    RecoverableAndPurgeable
}

#[derive(Debug)]
pub struct Vault<'a> {
    name: &'a str,
    token: &'a str,
}

impl Vault<'_> {
    fn proto(&self, resource: &str) -> Result<Builder, InvalidUri> {
        let uri: Uri = format!("{}{}?{}", self, resource, constants::API_VERSION)
            .parse()?;

        Ok(Request::builder()
            .uri(uri)
            .header("Authorization", format!("Bearer {}", self.token)))
    }

    pub fn secrets(&self) -> &impl secret::SecretVault {
        self
    }
}

impl fmt::Display for Vault<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "https://{}.vault.azure.net", self.name)
    }
}
