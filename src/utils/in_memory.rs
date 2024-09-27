#[derive(Default)]
pub struct InMemory {
    access_token: Option<String>,
    client_token: Option<String>,
}

impl InMemory {
    pub fn set_access_token(&mut self, value: String) { self.access_token = Some(value); }

    pub fn set_client_token(&mut self, value: String) { self.client_token = Some(value); }

    pub fn get_access_token(&self) -> Option<&String> { self.access_token.as_ref() }

    pub fn get_client_token(&self) -> Option<&String> { self.client_token.as_ref() }
}
