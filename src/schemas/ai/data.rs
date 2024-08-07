use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BasicAuth {
    pub password: String,
    pub username: String,
}

#[derive(Debug, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SshKeypair {
    pub private_key: String,
    pub public_key: Option<String>,
}

#[derive(Debug, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GitCredentials {
    pub basic_auth: Option<BasicAuth>,
    pub ssh_keypair: Option<SshKeypair>,
}

#[derive(Debug, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct S3Credentials {
    pub access_key: String,
    pub region: String,
    pub secret_key: String,
}

#[derive(Debug, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Credentials {
    pub git: Option<GitCredentials>,
    pub s3: Option<S3Credentials>,
}

#[derive(Debug, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FormDatastoreAlias {
    pub alias: String,
    pub credentials: Credentials,
    pub endpoint: Option<String>,
    pub owner: String, // customer┃ovhcloud
    pub prefix: Option<String>,
    pub r#type: String, // git┃s3┃swift
}

#[derive(Debug, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DatastoreAlias {
    pub alias: String,
    pub endpoint: Option<String>,
    pub owner: String, // customer┃ovhcloud
    pub prefix: Option<String>,
    pub r#type: String, // git┃s3┃swift
}
