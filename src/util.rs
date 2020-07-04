use crate::{BoxedResult, VaultError};

use bytes::buf::BufExt;
use hyper::{Body, Client, Request};
use hyper::body::aggregate;
use hyper_tls::HttpsConnector;
use percent_encoding::{NON_ALPHANUMERIC, utf8_percent_encode};
use serde::Deserialize;
use serde::de::DeserializeOwned;

#[inline]
pub fn encode_parameter(param: &str) -> String {
    utf8_percent_encode(param, NON_ALPHANUMERIC).to_string()
}

pub async fn slurp_json<T: DeserializeOwned>(req: Request<Body>) -> BoxedResult<T> {
    let client = Client::builder()
        .build(HttpsConnector::new());
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
