use std::sync::{
    Arc, 
    atomic::{AtomicBool, AtomicUsize, Ordering},
};

use crate::{
    backend::node,
  
};

#[derive(Debug,Default)]
pub struct RoundRobin{
    counter:AtomicUsize,
}

impl RoundRobin {
    pub fn new() -> Self{
        Self { counter: AtomicUsize::new(0), }
    }
    pub fn select_index(&self,backend_count: usize) -> Option<usize>{
        if backend_count == 0{
            return None;
        }

        let current  = self.counter.fetch_add(1, Ordering::Relaxed);
        Some(current % backend_count)
    }
    pub fn reset(&self) {
        self.counter.store(0,Ordering::Relaxed);
    }
    
}