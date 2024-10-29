use serde::{Deserialize, Serialize};

pub type RequestResult<T> = Result<YggdrasilResponse<T>, reqwest::Error>;

#[derive(Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum YggdrasilResponse<T = ()> {
    Success(T),
    Error(ErrorResponse),
}

#[derive(Serialize, Deserialize, Clone)]
pub struct AuthenticateResponse {
    #[serde(rename = "accessToken")]
    pub access_token: String,
    #[serde(rename = "clientToken")]
    pub client_token: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct ErrorResponse {
    pub error:         String,
    #[serde(rename = "errorMessage")]
    pub error_message: String,
}

#[derive(Serialize, Deserialize, Default, Clone)]
pub struct AuthenticateRequest {
    pub username: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct ProfileResponse {
    #[serde(rename = "name")]
    pub username: String,
    #[serde(rename = "id")]
    pub uuid:     String,
}
