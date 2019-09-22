use crate::core::Suggestion;
use crate::cache::Cache;
use itertools::Itertools;

pub struct PaxosCoordinator {
    nodes: Vec<Cache>
}

impl PaxosCoordinator {
    pub fn new(num_nodes: u32) -> PaxosCoordinator {
        let mut cache_nodes = Vec::new();
        for i in 0..num_nodes {
            cache_nodes.push(Cache::new(i, true))
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

    fn get_suggestion_for_set(&self, value: u32) -> Option<Suggestion> {
        for node in &self.nodes {
            match node.generate_suggestion(value) {
                Some(suggestion) => {
                    let num_permission_granted_nodes = self.nodes.iter().filter_map(|node| node.request(suggestion).ok()).count();
                    if num_permission_granted_nodes < self.responses_required_for_quorum() {
                        continue;
                    }
                    return Some(suggestion);
                }
                None => { }
            }
        }
        return None;
    }

    pub fn set(&mut self, value: u32) -> Result<(), ()> {
        match self.get_suggestion_for_set(value) {
            Some(suggestion) => {
                let num_suggestions_rejected = self.nodes.iter_mut().filter_map(|node| node.suggest(suggestion).err()).count();
                if num_suggestions_rejected < self.responses_required_for_quorum() {
                    return Ok(());
                }
                return Err(());

            },
            None => Err(())
        }
    }

    pub fn get(&self) -> Option<u32> {
        let mut responses = Vec::new();
        for cache in &self.nodes {
            responses.push(cache.get());
        }

        let mut current_max_response_count = 0;
        let mut current_max_response: Option<u32> = None;
        for (response, group) in &self.nodes.iter()
                .filter_map(|cache| cache.get())
                .group_by(|r| *r) {
            let response_counts = group.count();
            if response_counts > current_max_response_count {
                current_max_response = Some(response);
                current_max_response_count = response_counts;
            }
        }

        if current_max_response_count >= self.responses_required_for_quorum() { 
            current_max_response
        } 
        else {
            None
        }
    }

    pub fn set_cache_online_status(&mut self, cache_id: u32, status: bool) {
        println!("Cache {} online status set to {}", cache_id, status);
        match self.nodes.iter_mut().find(|node| node.cache_id == cache_id) {
            Some(cache) => cache.set_online_status(status),
            None => panic!(format!("Tried to set cache status to {} for cache id {} which does not exist.", status, cache_id))
        };
    }

    fn responses_required_for_quorum(&self) -> usize {
        (self.nodes.len() as f32 / 2.0).ceil() as usize
    }
}
