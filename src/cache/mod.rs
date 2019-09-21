use crate::core::{Suggestion, Accept, Nack, PermissionGranted};

use std::collections::HashMap;

pub struct Cache {
    value_map: HashMap<u32, u32>,
    cache_id: u32,
    last_accepted_suggestion: Option<Suggestion>
}

impl Cache {

    pub fn new(id: u32) -> Cache {
        let hashmap = HashMap::new();
        Cache {
            value_map: hashmap,
            cache_id: id,
            last_accepted_suggestion: None
        }
    }

    pub fn get(&self, key: u32) -> Option<u32> {
        if self.value_map.contains_key(&key) {
            Some(self.value_map[&key])
        }
        else {
            None
        }
    }

    pub fn generate_suggestion(&self, key: u32, proposed_value: u32) -> Suggestion {
        let last_suggestion_id = match self.last_accepted_suggestion {
            None => 0,
            Some(val) => val.id
        };
        Suggestion {
            id: last_suggestion_id + 1,
            cache_id: self.cache_id,
            key: key,
            value: proposed_value
        }
    }

    pub fn suggest(&mut self, suggestion: Suggestion) -> Result<Accept, Nack> {
        let accept = Accept {
            suggestion: suggestion
        };
        match self.get_nack_if_suggestion_is_old(suggestion) {
            None => {
                self.last_accepted_suggestion = Some(suggestion);
                Ok(accept)
            }
            Some(nack) => Err(nack)
        }
    }

    pub fn request(&mut self, suggestion: Suggestion) -> Result<PermissionGranted, Nack> {
        let last_accepted_value = self.get(suggestion.key);
        let permission_granted = PermissionGranted {
            corresponding_suggestion: suggestion,
            last_accepted_suggestion: self.last_accepted_suggestion,
            last_accepted_value: last_accepted_value
        };

        match self.get_nack_if_suggestion_is_old(suggestion) {
            None => Ok(permission_granted),
            Some(nack) => Err(nack)
        }
    }


    fn get_nack_if_suggestion_is_old(&self, suggestion: Suggestion)-> Option<Nack> {
        match self.last_accepted_suggestion {
            None => None,
            Some(val) => {
                if val.id > suggestion.id ||
                   (val.id == suggestion.id &&
                    val.cache_id > suggestion.cache_id) {
                    Some(Nack {
                        suggestion: val
                    })
                }
                else {
                    None
                }

            }
        }
    }
}