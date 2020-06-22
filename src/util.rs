use crate::BoxedResult;
use crate::error::VaultError;

use bytes::Buf;
use bytes::buf::BufExt;
use hyper::{Body, Client, Request};
use hyper::body::aggregate;
use hyper::client::HttpConnector;
use hyper_tls::HttpsConnector;
use serde::Deserialize;
use serde::de::DeserializeOwned;

pub fn https_client() -> Client<HttpsConnector<HttpConnector>> {
    Client::builder()
        .build(HttpsConnector::new())
}

async fn resolve_request(req: Request<Body>) -> BoxedResult<impl Buf> {
    let client = https_client();
    let res = client.request(req)
        .await?;

    let status = res.status();
    let body = aggregate(res).await?;
    if status.is_success() {
        Ok(body)
    }
    else {
        let mut de = serde_json::Deserializer::from_reader(body.reader());
        let e = VaultError::deserialize(&mut de)?;
        Err(Box::from(e))
    }
}

pub async fn slurp_error(req: Request<Body>) -> BoxedResult<()>{
    resolve_request(req)
        .await?;

    Ok(())
}

pub async fn slurp_json<T: DeserializeOwned>(req: Request<Body>) -> BoxedResult<T> {
    let buf = resolve_request(req)
        .await?;

    Ok(serde_json::from_reader(buf.reader())?)
}
