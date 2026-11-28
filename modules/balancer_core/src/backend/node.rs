use std::sync::{
    atomic::{AtomicU8,AtomicUsize,Ordering},
    Arc
};
use xxhash_rust::xxh3::xxh3_64;
use crate::backend::state::BackendState;


pub struct BackendNode{
    pub id: u64, 
    pub addr: String,
    pub weight: u32,
    pub active_connections: AtomicUsize,
    pub state: AtomicU8,
}
impl BackendNode {
    pub fn new(addr: String, weight: u32) -> Self
    {
        let id = xxh3_64(addr.as_bytes()); // deterministic hashing
        Self
         {     id,
               addr,
               weight,
               active_connections: AtomicUsize::new(0), 
               state: AtomicU8::new(BackendState::Healthy as u8) 
         }

    }
    
    // this performs checks to determine whether the backend is ready to accept incoming 
    //server requests
    pub fn is_healthy(&self) -> bool {
        BackendState::from(self.state.load(Ordering::Acquire)) == BackendState::Healthy
    }

    pub fn addr(&self) -> &str{
        &self.addr
    }

    pub fn active_connections(&self) -> usize {
        self.active_connections.load(Ordering::Relaxed)
    }
}




pub struct ConnectionGuard(pub Arc<BackendNode>);

impl ConnectionGuard {
    pub fn new(node: Arc<BackendNode>) -> Self{
        node.active_connections.fetch_add(1, Ordering::Relaxed);
        Self(node)
    }
    
}

impl Drop  for ConnectionGuard {
    fn drop(&mut self) {
        self.0.active_connections.fetch_sub(1, Ordering::Relaxed);
    }
    
}