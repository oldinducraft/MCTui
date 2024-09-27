pub mod types;

use std::time::Duration;

use reqwest::{Client, RequestBuilder};
use serde::Serialize;
use types::{AuthenticateRequest, AuthenticateResponse, RequestResult, YggdrasilResponse};

pub struct Yggdrasil {
    client:   Client,
    base_url: String,
}

impl Yggdrasil {
    pub fn new(base_url: String) -> Self {
        Self {
            client: Client::builder().timeout(Duration::from_secs(10)).build().unwrap(),
            base_url,
        }
    }

    pub async fn authenticate(&self, auth: AuthenticateRequest) -> RequestResult<AuthenticateResponse> {
        let result = self.post("auth/authenticate", auth).send().await?.text().await?;

        let response = serde_json::from_str::<YggdrasilResponse<AuthenticateResponse>>(&result).unwrap();
        Ok(response)
    }

    fn post<T: Serialize>(&self, path: &str, body: T) -> RequestBuilder {
        self.client.post(format!("{}/{}", self.base_url, path)).json(&body)
    }
}
