use std::sync::RwLock;

pub struct InMemory {
    inner: RwLock<InMemoryInner>,
}

impl InMemory {
    pub fn new() -> Self {
        Self {
            inner: RwLock::new(InMemoryInner::default()),
        }
    }

    pub fn get_access_token(&self) -> Option<String> { self.inner.read().unwrap().access_token.clone() }

    pub fn get_client_token(&self) -> Option<String> { self.inner.read().unwrap().client_token.clone() }

    pub fn get_username(&self) -> Option<String> { self.inner.read().unwrap().username.clone() }

    pub fn get_uuid(&self) -> Option<String> { self.inner.read().unwrap().uuid.clone() }

    pub fn set_access_token(&self, value: String) { self.inner.write().unwrap().access_token = Some(value); }

    pub fn set_client_token(&self, value: String) { self.inner.write().unwrap().client_token = Some(value); }

    pub fn set_username(&self, value: String) { self.inner.write().unwrap().username = Some(value); }

    pub fn set_uuid(&self, value: String) { self.inner.write().unwrap().uuid = Some(value); }
}

#[derive(Default)]
struct InMemoryInner {
    access_token: Option<String>,
    client_token: Option<String>,
    username:     Option<String>,
    uuid:         Option<String>,
}
