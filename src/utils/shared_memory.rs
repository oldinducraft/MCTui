use std::sync::{RwLock, RwLockReadGuard, RwLockWriteGuard};

pub struct SharedMemory {
    inner: RwLock<SharedMemoryInner>,
}

impl SharedMemory {
    pub fn new() -> Self {
        Self {
            inner: RwLock::new(SharedMemoryInner::default()),
        }
    }

    pub fn get_access_token(&self) -> Option<String> { self.read().access_token.clone() }

    pub fn get_username(&self) -> Option<String> { self.read().username.clone() }

    pub fn get_uuid(&self) -> Option<String> { self.read().uuid.clone() }

    pub fn set_access_token(&self, value: String) { self.write().access_token = Some(value); }

    pub fn set_username(&self, value: String) { self.write().username = Some(value); }

    pub fn set_uuid(&self, value: String) { self.write().uuid = Some(value); }

    fn read(&self) -> RwLockReadGuard<SharedMemoryInner> {
        self.inner
            .read()
            .unwrap_or_else(|err| panic!("Failed to lock SharedMemory (read): {}", err))
    }

    fn write(&self) -> RwLockWriteGuard<SharedMemoryInner> {
        self.inner
            .write()
            .unwrap_or_else(|err| panic!("Failed to lock SharedMemory (write): {}", err))
    }

    pub fn auth_args_are_set(&self) -> bool {
        self.read().access_token.is_some() && self.read().username.is_some() && self.read().uuid.is_some()
    }
}

#[derive(Default)]
struct SharedMemoryInner {
    access_token: Option<String>,
    username:     Option<String>,
    uuid:         Option<String>,
}
