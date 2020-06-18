use crate::{JSONResult, Vault};
use crate::constants;

use bytes::buf::BufExt;
use hyper::{Body, Client, Request, Uri};
use hyper::client::HttpConnector;
use hyper::http::request::Builder;
use hyper::http::uri::InvalidUri;
use hyper_tls::HttpsConnector;
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

pub async fn slurp_json<T: DeserializeOwned>(req: Request<Body>) -> JSONResult<T>
{
    let client = https_client();
    let res = client.request(req)
        .await?;
    let body = hyper::body::aggregate(res)
        .await?;
    Ok(serde_json::from_reader(body.reader())?)
}
