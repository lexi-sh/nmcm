use kv::{Config, Error, Manager, ValueRef};

mod bag {
    struct Cache {
        manager: Manager
    }

    impl Cache {
        pub fn set (&self, key: String, value: Vec<u8>) {
            
        }
    }
}