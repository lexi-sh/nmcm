use std::collections::HashMap;

pub struct Cache {
    string_key_value_map: HashMap<String, u32>,
}

impl Cache {

    pub fn new() -> Cache {
        let hashmap = HashMap::new();
        Cache {
            string_key_value_map: hashmap
        }
    }

    pub fn set (&mut self, key: String, value: u32) -> Option<u32> {
        self.string_key_value_map.insert(key, value)
    }

    pub fn get (&mut self, key: String) -> Option<&u32> {
        self.string_key_value_map.get(&key)
    }
}