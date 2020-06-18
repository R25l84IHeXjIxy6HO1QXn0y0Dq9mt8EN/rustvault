use crate::{JSONResult, Vault};
use crate::util::{slurp_json, vault_request_proto};

use hyper::Body;
use serde_json::{json, Value};

/*
TODO:
- backup_secret
- purge_deleted_secret
- recover_deleted_secret
- restore_secret
- update_secret
*/

pub async fn get_secrets(vault: Vault<'_>) -> JSONResult<Value> {
    let resource_name = "/secrets";

    let req = vault_request_proto(vault, &resource_name)?
        .method("GET")
        .body(Body::empty())
        .unwrap();

    slurp_json(req)
        .await
}

pub async fn get_secret_versions(vault: Vault<'_>, secret_name: &str) -> JSONResult<Value> {
    let resource_name = format!("/secrets/{}", secret_name);

    let req = vault_request_proto(vault, &resource_name)?
        .method("GET")
        .body(Body::empty())
        .unwrap();

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
        .body(Body::empty())
        .unwrap();

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
        .body(Body::from(serde_json::to_string(&payload)?))
        .unwrap();

    slurp_json(req)
        .await
}
