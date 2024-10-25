use std::sync::Arc;

use crate::utils::immediate_rw_lock::ImmediateRwLock;

#[derive(Default)]
pub struct ProgressState {
    pub(super) percent: Arc<ImmediateRwLock<f64>>,
}

impl ProgressState {
    pub fn try_set(&self, percent: f64) { let _ = self.percent.try_set(percent); }
}
