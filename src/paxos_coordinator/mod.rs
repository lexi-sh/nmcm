use crate::cache::Cache;
use std::collections::HashMap;

pub struct PaxosCoordinator {
    nodes: Vec<Cache>
}

impl PaxosCoordinator {
    pub fn new(num_nodes: u32) -> PaxosCoordinator {
        let mut cache_nodes = Vec::new();
        for i in 0..num_nodes {
            cache_nodes.push(Cache::new(i))
        }
        PaxosCoordinator {
            nodes: cache_nodes
        }
    }

    pub fn with_nodes(nodes: Vec<Cache>) -> PaxosCoordinator {
        PaxosCoordinator {
            nodes: nodes
        }
    }

    pub fn get(&self, key: u32) -> Option<u32> {
        let mut responses = Vec::new();
        for cache in &self.nodes {
            responses.push(cache.get(key));
        }

        let response_counts: HashMap<u32, u32> = HashMap::new();
        None
    }

    fn responses_required_for_quorum(&self) -> u32 {
        (self.nodes.len() as f32 / 2.0).ceil() as u32
    }
}
