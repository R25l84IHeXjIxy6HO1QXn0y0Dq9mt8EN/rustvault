use crate::{BoxedResult, Vault, VaultError};
use crate::constants;

use bytes::buf::BufExt;
use hyper::{Body, Client, Request, Uri};
use hyper::body::aggregate;
use hyper::client::HttpConnector;
use hyper::http::request::Builder;
use hyper::http::uri::InvalidUri;
use hyper_tls::HttpsConnector;
use serde::Deserialize;
use serde::de::DeserializeOwned;

pub fn https_client() -> Client<HttpsConnector<HttpConnector>> {
    Client::builder()
        .build(HttpsConnector::new())
}

pub fn vault_request_proto(v: Vault, resource_name: &str) ->  Result<Builder, InvalidUri> {
    let uri: Uri = format!("{}{}?{}", v, resource_name, constants::API_VERSION)
        .parse()?;
    Ok(Request::builder()
        .uri(uri)
        .header("Authorization", format!("Bearer {}", v.token)))
}

pub async fn slurp_json<T: DeserializeOwned>(req: Request<Body>) -> BoxedResult<T> {
    let client = https_client();
    let res = client.request(req)
        .await?;

    let status = res.status();
    let body = aggregate(res).await?;
    if status.is_success() {
        Ok(serde_json::from_reader(body.reader())?)
    }
    else {
        let mut de = serde_json::Deserializer::from_reader(body.reader());
        let e = VaultError::deserialize(&mut de)?;
        Err(Box::from(e))
    }
}
