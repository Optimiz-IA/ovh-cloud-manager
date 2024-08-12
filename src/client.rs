use once_cell::sync::Lazy;
use reqwest::header::{HeaderMap, HeaderValue};
use reqwest::Url;
use serde::Serialize;
use sha1::{Digest, Sha1};
use std::collections::HashMap;
use std::sync::RwLock;
use std::time::{SystemTime, UNIX_EPOCH};

use crate::errors::OvhManagerError;

static ENDPOINTS: Lazy<HashMap<&'static str, &'static str>> = Lazy::new(|| {
    HashMap::from([
        ("ovh-eu", "https://eu.api.ovh.com/1.0"),
        ("ovh-us", "https://api.us.ovhcloud.com/1.0"),
        ("ovh-ca", "https://ca.api.ovh.com/1.0"),
        ("kimsufi-eu", "https://eu.api.kimsufi.com/1.0"),
        ("kimsufi-ca", "https://ca.api.kimsufi.com/1.0"),
        ("soyoustart-eu", "https://eu.api.soyoustart.com/1.0"),
        ("soyoustart-ca", "https://ca.api.soyoustart.com/1.0"),
    ])
});

fn get_system_time() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
}

fn insert_sensitive_header(headers: &mut HeaderMap, header_name: &'static str, value: &str) {
    let mut header_value = HeaderValue::from_str(value).unwrap();
    header_value.set_sensitive(true);
    headers.insert(header_name, header_value);
}

pub struct OvhClient {
    endpoint: &'static str,
    application_key: String,
    application_secret: String,
    consumer_key: String,
    http_client: reqwest::Client,
    time_delta: RwLock<Option<u64>>,
}

impl OvhClient {
    pub fn new(
        endpoint: &str,
        application_key: &str,
        application_secret: &str,
        consumer_key: &str,
    ) -> Result<Self, OvhManagerError> {
        let endpoint = match ENDPOINTS.get(endpoint) {
            Some(value) => value,
            None => return Err(OvhManagerError::EndpointNotFound),
        };

        Ok(Self {
            endpoint,
            application_key: application_key.to_string(),
            application_secret: application_secret.to_string(),
            consumer_key: consumer_key.to_string(),
            http_client: reqwest::Client::new(),
            time_delta: RwLock::new(None),
        })
    }

    fn format_url(&self, path: &str) -> String {
        format!("{}{}", self.endpoint, path)
    }

    pub async fn get_server_time(&self) -> Result<u64, OvhManagerError> {
        let headers = self.create_base_headers();

        let url = self.format_url("/auth/time");
        let response = self
            .http_client
            .get(url)
            .headers(headers)
            .send()
            .await?
            .text()
            .await?;
        let time: u64 = response.parse()?;

        Ok(time)
    }

    pub async fn compute_time_delta(&self) -> Result<u64, OvhManagerError> {
        let server_time = self.get_server_time().await?;
        let system_time = get_system_time();

        if server_time < system_time {
            Ok(0)
        } else {
            Ok(server_time - system_time)
        }
    }

    pub fn signature(&self, url: &str, timestamp: &str, method: &str, body: &str) -> String {
        let values = [
            &self.application_secret,
            &self.consumer_key,
            method,
            url,
            body,
            timestamp,
        ];

        let mut hasher = Sha1::new();
        hasher.update(values.join("+"));

        format!("$1${:x}", hasher.finalize())
    }

    fn create_base_headers(&self) -> HeaderMap {
        let mut headers = HeaderMap::new();
        headers.insert(
            "Content-Type",
            HeaderValue::from_str("application/json").unwrap(),
        );
        headers.insert(
            "X-Ovh-Application",
            HeaderValue::from_str(&self.application_key).unwrap(),
        );

        headers
    }

    pub async fn create_headers(
        &self,
        method: &str,
        url: &str,
        body: &str,
    ) -> Result<HeaderMap, OvhManagerError> {
        let mut headers = self.create_base_headers();

        if self.time_delta.read().unwrap().is_none() {
            let time_delta = self.compute_time_delta().await?;
            // self.time_delta.set(Some(time_delta));
            let mut w = self.time_delta.write().unwrap();
            *w = Some(time_delta);
        }

        let now = get_system_time();
        let timestamp = now + self.time_delta.read().unwrap().unwrap();
        let timestamp = timestamp.to_string();

        let signature = self.signature(url, &timestamp, method, body);

        insert_sensitive_header(&mut headers, "X-Ovh-Consumer", &self.consumer_key);
        insert_sensitive_header(&mut headers, "X-Ovh-Timestamp", &timestamp);
        insert_sensitive_header(&mut headers, "X-Ovh-Signature", &signature);

        Ok(headers)
    }

    pub async fn get(
        &self,
        path: &str,
        parameters: Option<&[(&str, &str)]>,
    ) -> Result<reqwest::Response, OvhManagerError> {
        let mut url = self.format_url(path);

        if let Some(values) = parameters {
            url = match Url::parse_with_params(&url, values) {
                Ok(value) => value.to_string(),
                Err(_) => return Err(OvhManagerError::ParseUrlError),
            }
        }

        let headers = self.create_headers("GET", &url, "").await?;
        let response = self.http_client.get(url).headers(headers).send().await?;

        Ok(response)
    }

    pub async fn delete(
        &self,
        path: &str,
        parameters: Option<&[(&str, &str)]>,
    ) -> Result<reqwest::Response, OvhManagerError> {
        let mut url = self.format_url(path);

        if let Some(values) = parameters {
            url = match Url::parse_with_params(&url, values) {
                Ok(value) => value.to_string(),
                Err(_) => return Err(OvhManagerError::ParseUrlError),
            }
        }

        let headers = self.create_headers("DELETE", &url, "").await?;
        let response = self.http_client.delete(url).headers(headers).send().await?;

        Ok(response)
    }

    pub async fn post<T: Serialize>(
        &self,
        path: &str,
        data: &T,
    ) -> Result<reqwest::Response, OvhManagerError> {
        let url = self.format_url(path);
        let body = serde_json::to_string(data)?;
        let headers = self.create_headers("POST", &url, &body).await?;

        let response = self
            .http_client
            .post(url)
            .headers(headers)
            .body(body)
            .send()
            .await?;

        Ok(response)
    }

    pub async fn put<T: Serialize>(
        &self,
        path: &str,
        data: Option<&T>,
    ) -> Result<reqwest::Response, OvhManagerError> {
        let url = self.format_url(path);
        let body = match data {
            Some(value) => serde_json::to_string(value)?,
            None => String::new(),
        };
        let headers = self.create_headers("PUT", &url, &body).await?;

        let response = self
            .http_client
            .put(url)
            .headers(headers)
            .body(body)
            .send()
            .await?;

        Ok(response)
    }
}
