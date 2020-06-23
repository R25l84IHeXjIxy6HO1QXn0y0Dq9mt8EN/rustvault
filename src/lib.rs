mod constants;
mod util;

use util::*;

use std::fmt;

use hyper::Body;
use serde_json::{json, Value};

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


pub async fn get_secrets(vault: Vault<'_>) -> JSONResult<Value> {
    let resource_name = "/secrets";

    let req = vault_request_proto(vault, &resource_name)?
        .method("GET")
        .body(Body::empty())?;

    slurp_json(req)
        .await
}

pub async fn get_secret_versions(vault: Vault<'_>, secret_name: &str) -> JSONResult<Value> {
    let resource_name = format!("/secrets/{}", secret_name);

    let req = vault_request_proto(vault, &resource_name)?
        .method("GET")
        .body(Body::empty())?;

    slurp_json(req)
        .await
}

pub async fn get_secret(vault: Vault<'_>, secret_name: &str, secret_version: &str) -> JSONResult<Value> {
    let resource_name = format!(
        "/secrets/{}/version/{}",
        secret_name, secret_version
    );

    let req = vault_request_proto(vault, &resource_name)?
        .method("GET")
        .body(Body::empty())?;

    slurp_json(req)
        .await
}

pub async fn set_secret(vault: Vault<'_>, secret_name: &str, secret_value: &str) -> JSONResult<Value> {
    let resource_name = format!("/secrets/{}", secret_name);
    let payload = json!({
        "value": secret_value
    });

    let req = vault_request_proto(vault, &resource_name)?
        .method("PUT")
        .body(Body::from(serde_json::to_string(&payload)?))?;

    slurp_json(req)
        .await
}
