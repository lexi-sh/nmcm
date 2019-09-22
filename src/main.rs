pub mod cache;
pub mod core;
pub mod paxos_coordinator;

use crate::cache::Cache;
use crate::paxos_coordinator::PaxosCoordinator;

fn main() {
    println!("Paxos examples started");
    happy_path();
    println!("================================================================================");
    node_catch_up();
}

fn happy_path() {
    let cache_one = Cache::new(1, true);
    let cache_two = Cache::new(2, true);
    let cache_three = Cache::new(3, true);

    let mut coordinator = PaxosCoordinator::with_nodes(vec![cache_one, cache_two, cache_three]);
    
    let set_response = coordinator.set(4);
    assert!(set_response.is_ok());
    
    let receiceved_response = coordinator.get();
    assert!(receiceved_response.is_some());
    match receiceved_response {
        Some(val) => assert_eq!(4, val),
        None => assert!(false)
    }
}

#[allow(unused_must_use)]
fn node_catch_up() {
    let cache_one = Cache::new(1, true);
    let cache_two = Cache::new(2, true);
    let cache_three = Cache::new(3, true);

    let mut coordinator = PaxosCoordinator::with_nodes(vec![cache_one, cache_two, cache_three]);
    coordinator.set(1);
    coordinator.set(2);
    coordinator.set_cache_online_status(1, false);
    coordinator.set(5);
    coordinator.set(6);
    assert_value_is_some(coordinator.get(), 6);
    coordinator.set_cache_online_status(1, true);
    assert_value_is_some(coordinator.get(), 6);
    coordinator.set(7);
    assert_value_is_some(coordinator.get(), 7);   
}

fn assert_value_is_some(cache_value: Option<u32>, expected_value: u32) {
    assert!(cache_value.is_some());
    match cache_value {
        Some(val) => assert_eq!(expected_value, val),
        None => assert!(false)
    };
}