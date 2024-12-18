use std::{
    sync::Arc,
    time::{Duration, Instant},
};

use bon::Builder;
use miette::Diagnostic;
use reqwest::{
    header::{HeaderMap, HeaderValue, COOKIE},
    Client, Url,
};
use thiserror::Error;

pub const DEFAULT_BASE_URL: &str = "https://www.boomlings.com/database";

#[derive(Debug, Clone, Builder)]
pub struct Http {
    pub client: Client,
    pub base_url: Arc<Url>,
}

pub const GD: HeaderValue = HeaderValue::from_static("gd=1");

pub fn default_headers() -> HeaderMap<HeaderValue> {
    let mut headers = HeaderMap::new();

    headers.append(COOKIE, GD);

    headers
}

pub fn default_base_url() -> Url {
    Url::parse(DEFAULT_BASE_URL).unwrap()
}

pub fn default_client() -> Result<Client> {
    let client = Client::builder()
        .default_headers(default_headers())
        .build()?;

    Ok(client)
}

impl Default for Http {
    fn default() -> Self {
        Self::create().expect("failed to create the client")
    }
}

impl Http {
    pub fn new(client: Client) -> Self {
        let base_url = Arc::new(default_base_url());

        Self { client, base_url }
    }

    pub fn create() -> Result<Self, CreateError> {
        default_client().map(Self::new)
    }

    pub async fn ping(&self) -> Result<Duration, PingError> {
        let instant = Instant::now();

        self.client
            .get(self.base_url)
            .send()
            .await
            .map_err(PingError)?;

        Ok(instant.elapsed())
    }
}
