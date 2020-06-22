use crate::{BoxedResult, DeletionRecoveryLevel, Vault};
use crate::util::{slurp_error, slurp_json};

use anyof_struct::anyof;
use async_trait::async_trait;
use hyper::Body;
use serde::{Deserialize, Serialize};
use serde_json::{json, to_string, Value};

#[anyof(compact)]
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Attributes {
    created: u64,
    enabled: bool,
    exp: u64,
    nbf: u64,
    recovery_level: DeletionRecoveryLevel,
    updated: u64
}

#[async_trait]
pub trait SecretVault<'a> {
    async fn backup(&self, name: &str) -> BoxedResult<Value>;
    async fn list(&self) -> BoxedResult<Value>;
    async fn get_versions(&self, name: &str) -> BoxedResult<Value>;
    async fn get(&self, name: &str, version: &str) -> BoxedResult<Value>;
    async fn purge(&self, name: &str) -> BoxedResult<()>;
    async fn recover(&self, name: &str) -> BoxedResult<Value>;
    async fn restore(&self, value: &str) -> BoxedResult<Value>;
    async fn set(&self, name: &str, value: &str) -> BoxedResult<Value>;
    async fn update(&self, name: &str, version: &str, attrs: Option<Attributes>, content_type: Option<&str>, tags: Option<Value>) -> BoxedResult<Value>;
}

#[async_trait]
impl<'a> SecretVault<'a> for Vault<'a> {
    async fn backup(&self, name: &str) -> BoxedResult<Value> {
        let resource_name = format!("/secrets/{}/backup", name);

        let req = self.proto(&resource_name)?
            .method("POST")
            .body(Body::empty())?;
            
        slurp_json(req)
            .await
    }

    async fn list(&self) -> BoxedResult<Value> {
        let resource_name = "/secrets";

        let req = self.proto(&resource_name)?
            .method("GET")
            .body(Body::empty())?;

        slurp_json(req)
            .await
    }

    async fn get_versions(&self, name: &str) -> BoxedResult<Value> {
        let resource_name = format!("/secrets/{}", name);

        let req = self.proto(&resource_name)?
            .method("GET")
            .body(Body::empty())?;

        slurp_json(req)
            .await
    }

    async fn get(&self, name: &str, version: &str) -> BoxedResult<Value> {
        let resource_name = format!("/secrets/{}/{}", name, version);

        let req = self.proto(&resource_name)?
            .method("GET")
            .body(Body::empty())?;

        slurp_json(req)
            .await
    }

    async fn purge(&self, name: &str) -> BoxedResult<()> {
        let resource_name = format!("/deletedsecrets/{}", name);

        let req = self.proto(&resource_name)?
            .method("DELETE")
            .body(Body::empty())?;

        slurp_error(req)
            .await
    }

    async fn recover(&self, name: &str) -> BoxedResult<Value> {
        let resource_name = format!("/secrets/deletedsecrets/{}/recover", name);

        let req = self.proto(&resource_name)?
            .method("POST")
            .body(Body::empty())?;

        slurp_json(req)
            .await
    }

    async fn restore(&self, value: &str) -> BoxedResult<Value> {
        let resource_name = format!("/secrets/restore");
        let payload = json!({
            "value": value
        });

        let req = self.proto(&resource_name)?
            .method("GET")
            .body(Body::from(to_string(&payload)?))?;

        slurp_json(req)
            .await
    }

    async fn set(&self, name: &str, value: &str) -> BoxedResult<Value> {
        let resource_name = format!("/secrets/{}", name);
        let payload = json!({
            "value": value
        });

        let req = self.proto(&resource_name)?
            .method("PUT")
            .body(Body::from(serde_json::to_string(&payload)?))?;

        slurp_json(req)
            .await
    }

    async fn update(&self, name: &str, version: &str, attrs: Option<Attributes>, content_type: Option<&str>, tags: Option<Value>) -> BoxedResult<Value> {
        let resource_name = format!("/secrets/{}/{}", name, version);
        let payload = json!({
            "attributes": attrs,
            "contentType": content_type,
            "tags": tags
        });

        let req = self.proto(&resource_name)?
            .method("PATCH")
            .body(Body::from(serde_json::to_string(&payload)?))?;

        slurp_json(req)
            .await
    }
}
