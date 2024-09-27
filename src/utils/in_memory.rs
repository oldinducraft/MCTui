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

    pub fn get_access_token(&self) -> Option<String> {
        let inner = self.inner.read().unwrap();
        inner.access_token.clone()
    }

    pub fn get_client_token(&self) -> Option<String> {
        let inner = self.inner.read().unwrap();
        inner.client_token.clone()
    }

    pub fn set_access_token(&self, value: String) {
        let mut inner = self.inner.write().unwrap();
        inner.access_token = Some(value);
    }

    pub fn set_client_token(&self, value: String) {
        let mut inner = self.inner.write().unwrap();
        inner.client_token = Some(value);
    }
}

#[derive(Default)]
struct InMemoryInner {
    access_token: Option<String>,
    client_token: Option<String>,
}
