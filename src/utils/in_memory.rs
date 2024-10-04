use std::sync::{RwLock, RwLockReadGuard, RwLockWriteGuard};

pub struct InMemory {
    inner: RwLock<InMemoryInner>,
}

impl InMemory {
    pub fn new() -> Self {
        Self {
            inner: RwLock::new(InMemoryInner::default()),
        }
    }

    pub fn get_access_token(&self) -> Option<String> { self.read().access_token.clone() }

    pub fn get_client_token(&self) -> Option<String> { self.read().client_token.clone() }

    pub fn get_username(&self) -> Option<String> { self.read().username.clone() }

    pub fn get_uuid(&self) -> Option<String> { self.read().uuid.clone() }

    pub fn set_access_token(&self, value: String) { self.write().access_token = Some(value); }

    pub fn set_client_token(&self, value: String) { self.write().client_token = Some(value); }

    pub fn set_username(&self, value: String) { self.write().username = Some(value); }

    pub fn set_uuid(&self, value: String) { self.write().uuid = Some(value); }

    fn read(&self) -> RwLockReadGuard<InMemoryInner> { self.inner.read().unwrap() }

    fn write(&self) -> RwLockWriteGuard<InMemoryInner> { self.inner.write().unwrap() }

    pub fn auth_args_are_set(&self) -> bool {
        self.read().access_token.is_some() &&
            self.read().client_token.is_some() &&
            self.read().username.is_some() &&
            self.read().uuid.is_some()
    }
}

#[derive(Default)]
struct InMemoryInner {
    access_token: Option<String>,
    client_token: Option<String>,
    username:     Option<String>,
    uuid:         Option<String>,
}
