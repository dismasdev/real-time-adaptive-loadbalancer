use crate::backend::node;
use std::sync::Arc;

pub trait BalancingStrategy: Send + Sync {
    fn select(&self, backends: &[Arc<BackendNode>]) -> Result<Arc<BackendNode>, BackendError> {}
}
