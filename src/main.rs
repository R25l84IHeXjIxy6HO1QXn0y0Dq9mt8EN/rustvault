use std::fs::File;

use serde_json::Value;

mod constants;
mod secret;

#[tokio::main]
async fn main() {
    let token_file: Value = {
        let contents = File::open("./token.json")
            .unwrap();
        serde_json::from_reader(contents)
            .unwrap()
    };
    let access_token = token_file["accessToken"]
        .as_str()
        .unwrap();
    let vault: secret::Vault = secret::Vault {
        name: "vaultdummy",
        token: access_token
    };
    match secret::get_secret_versions(vault, "foo").await {
        Ok(json) => println!("{}", json),
        Err(err) => println!("ERROR: {}", err)
    }
}
