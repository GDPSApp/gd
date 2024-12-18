// use std::time::Duration;

// use miette::Diagnostic;
// use thiserror::Error;

use std::time::Duration;

use crate::internals::http::Http;

#[derive(Debug, Clone, Default)]
pub struct Session {
    pub http: Http,
}

impl Session {
    pub fn new(http: Http) -> Self {
        Self { http }
    }

    pub async fn ping(&self) -> Duration {
        self.http.ping().await
    }
}
