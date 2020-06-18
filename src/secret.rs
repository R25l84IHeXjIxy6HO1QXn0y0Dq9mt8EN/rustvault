use crate::constants;

use std::fmt;

use bytes::buf::BufExt;
use hyper::{Body, Client, Request, Uri};
use hyper::client::connect::HttpConnector;
use hyper::http::request::Builder;
use hyper_tls::HttpsConnector;
use serde_json::Value;

type JSONResult = Result<Value, Box<dyn std::error::Error>>;

#[derive(Debug)]
pub struct Vault<'a> {
    pub name: &'a str,
    pub token: &'a str
}

impl fmt::Display for Vault<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "https://{}.vault.azure.net", self.name)
    }
}

fn https_client() -> Client<HttpsConnector<HttpConnector>> {
    Client::builder()
        .build(HttpsConnector::new())
}

fn vault_request(vault: Vault, resource_name: &str) -> Result<Builder, Box<dyn std::error::Error>> {
    let uri: Uri = format!("{}{}", vault, resource_name)
        .parse()?;
    Ok(Request::builder()
        .uri(uri)
        .header("Authorization", format!("Bearer {}", vault.token)))
}

async fn slurp_json(req: Request<Body>) -> JSONResult {
    let client = https_client();
    let res = client.request(req)
        .await?;
    let body = hyper::body::aggregate(res)
        .await?;
    Ok(serde_json::from_reader(body.reader())?)
}

pub async fn get_secrets<'a>(vault: Vault<'a>) -> JSONResult {
    let resource_name = format!("/secrets?api-version={}", constants::API_VERSION);

    let req = vault_request(vault, &resource_name)?
        .method("GET")
        .body(Body::empty())
        .unwrap();

    slurp_json(req)
        .await
}

pub async fn get_secret_versions<'a>(vault: Vault<'a>, secret_name: &'a str) -> JSONResult {
    let resource_name = format!("/secrets/{}?api-version={}", secret_name, constants::API_VERSION);

    let req = vault_request(vault, &resource_name)?
        .method("GET")
        .body(Body::empty())
        .unwrap();

    slurp_json(req)
        .await
}

pub async fn get_secret<'a>(vault: Vault<'a>, secret_name: &'a str, secret_version: &'a str) -> JSONResult {
    let resource_name = format!(
        "/secrets/{}/version/{}?api-version={}",
        secret_name, secret_version, constants::API_VERSION
    );

    let req = vault_request(vault, &resource_name)?
        .method("GET")
        .body(Body::empty())
        .unwrap();

    slurp_json(req)
        .await
}
