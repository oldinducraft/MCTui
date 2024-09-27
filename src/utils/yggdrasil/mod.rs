pub mod types;

use std::time::Duration;

use reqwest::{Client, RequestBuilder};
use serde::Serialize;
use types::{AuthenticateRequest, AuthenticateResponse, RequestResult, YggdrasilResponse};

const YGGDRASIL_HOST: &str = "https://wayaway.asuscomm.com";

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

        let response = serde_json::from_str::<YggdrasilResponse<AuthenticateResponse>>(&result).unwrap();
        Ok(response)
    }

    fn post<T: Serialize>(&self, path: &str, body: T) -> RequestBuilder {
        self.client.post(format!("{}/{}", YGGDRASIL_HOST, path)).json(&body)
    }
}
