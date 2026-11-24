use std::sync::Arc;
use crate::backend::node;



pub trait BalancingStrategy: Send + Sync {
    fn select(&self,backends: &[Arc<BackendNode>],) -> Result<Arc<BackendNode>,BackendError>{
        

    }
    
}