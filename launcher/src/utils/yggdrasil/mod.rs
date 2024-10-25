pub mod types;

use std::time::Duration;

use reqwest::{Client, RequestBuilder};
use serde::Serialize;
use types::{AuthenticateRequest, AuthenticateResponse, ProfileResponse, RequestResult, YggdrasilResponse};

use constants::YGGDRASIL_DOMAIN;

pub struct Yggdrasil {
    client: Client,
}

impl Yggdrasil {
    pub fn new() -> Self {
        Self {
            client: Client::builder().timeout(Duration::from_secs(10)).build().unwrap(),
        }
    }

    pub async fn authenticate(&self, auth: AuthenticateRequest) -> RequestResult<AuthenticateResponse> {
        let result = self.post("auth/authenticate", auth).send().await?.text().await?;

        let response = serde_json::from_str::<YggdrasilResponse<AuthenticateResponse>>(&result)
            .unwrap_or_else(|err| panic!("Failed to parse authenticate response: {}", err));
        Ok(response)
    }

    pub async fn get_profile(&self, username: &str) -> RequestResult<ProfileResponse> {
        let result = self
            .get(&format!("account/users/profiles/minecraft/{username}"))
            .send()
            .await?
            .text()
            .await?;

        let response = serde_json::from_str::<YggdrasilResponse<ProfileResponse>>(&result)
            .unwrap_or_else(|err| panic!("Failed to parse profile response: {}", err));
        Ok(response)
    }

    fn post<T: Serialize>(&self, path: &str, body: T) -> RequestBuilder {
        self.client
            .post(format!("https://{}/{}", YGGDRASIL_DOMAIN, path))
            .json(&body)
    }

    fn get(&self, path: &str) -> RequestBuilder { self.client.get(format!("https://{}/{}", YGGDRASIL_DOMAIN, path)) }
}
