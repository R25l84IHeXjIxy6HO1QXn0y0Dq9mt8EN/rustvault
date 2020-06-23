mod constants;
mod util;

use util::*;

use std::error::Error;
use std::fmt;

use hyper::Body;
use percent_encoding::{NON_ALPHANUMERIC, utf8_percent_encode};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

pub type BoxedResult<T> = Result<T, Box<dyn std::error::Error>>;

#[derive(Debug)]
pub struct Vault<'a> {
    name: &'a str,
    token: &'a str
}

#[derive(Debug, Deserialize)]
pub struct VaultError<'a> {
    #[serde(borrow)]
    error: ErrorContainer<'a>
}

#[derive(Serialize)]
pub struct Key<'a> {
    id: &'a str,
    value: &'a str
}

#[derive(Debug, Deserialize)]
struct ErrorContainer<'a> {
    code: &'a str,
    message: &'a str,
    #[serde(rename(deserialize = "innererror"))]
    inner_error: Option<Box<ErrorContainer<'a>>>
}

impl fmt::Display for Vault<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "https://{}.vault.azure.net", self.name)
    }
}

impl fmt::Display for VaultError<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "VaultError({}): {}", self.error.code, self.error.message)
    }
}

impl Error for VaultError<'_> { }

#[inline]
fn encode_parameter(param: &str) -> String {
    utf8_percent_encode(param, NON_ALPHANUMERIC).to_string()
}

pub async fn get_secrets(vault: Vault<'_>) -> BoxedResult<Value> {
    let resource_name = "/secrets";

    let req = vault_request_proto(vault, &resource_name)?
        .method("GET")
        .body(Body::empty())?;

    slurp_json(req)
        .await
}

pub async fn get_secret_versions(vault: Vault<'_>, secret_name: &str) -> BoxedResult<Value> {
    let resource_name = format!(
        "/secrets/{}",
        encode_parameter(secret_name)
    );

    let req = vault_request_proto(vault, &resource_name)?
        .method("GET")
        .body(Body::empty())?;

    slurp_json(req)
        .await
}

pub async fn get_secret(vault: Vault<'_>, secret_name: &str, secret_version: Option<&str>) -> BoxedResult<Value> {
    let resource_name = format!(
        "/secrets/{}/version/{}",
        encode_parameter(secret_name),
        encode_parameter(secret_version.unwrap_or_default())
    );

    let req = vault_request_proto(vault, &resource_name)?
        .method("GET")
        .body(Body::empty())?;

    slurp_json(req)
        .await
}

pub async fn set_secret(vault: Vault<'_>, secret_name: &str, secret_value: &str) -> BoxedResult<Value> {
    let resource_name = format!(
        "/secrets/{}",
        encode_parameter(secret_name)
    );
    let payload = json!({
        "value": secret_value
    });

    let req = vault_request_proto(vault, &resource_name)?
        .method("PUT")
        .body(Body::from(serde_json::to_string(&payload)?))?;

    slurp_json(req)
        .await
}
