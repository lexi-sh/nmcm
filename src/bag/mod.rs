use std::collections::HashMap;

pub struct Cache {
    value_map: HashMap<u32, u32>,
    cache_id: u32,
    last_accepted_suggestion_id: Option<SuggestionId>
}

#[derive(Copy)]
struct SuggestionId {
    id: u32,
    cache_id: u32,
    key: u32,
    value: u32
}

struct PermissionGranted {
    corresponding_suggestion_id: SuggestionId,
    last_accepted_suggestion_id: Option<SuggestionId>,
    last_accepted_value: Option<u32>
}

struct Nack {
    last_accepted_suggestion_id: Option<SuggestionId>
}

struct Accepted {
    suggestion_id: SuggestionId
}

impl Cache {

    pub fn new(id: u32) -> Cache {
        let hashmap = HashMap::new();
        Cache {
            value_map: hashmap,
            cache_id: id,
            last_accepted_suggestion_id: None
        }
    }

    pub fn suggest(&self, suggestion_id: SuggestionId) -> Result<Accepted, Nack> {
        let accept = Accepted {
            suggestion_id: suggestion_id
        };
        let nack = Nack {
            last_accepted_suggestion_id: self.last_accepted_suggestion_id
        };
        if self.is_newer(suggestion_id) {
            self.last_accepted_suggestion_id = Some(suggestion_id);
            Ok(accept)
        }
        else {
            Err(nack)
        }
    }

    fn is_newer(&self, suggestion_id: SuggestionId) -> bool {
        match self.last_accepted_suggestion_id {
            None => true,
            Some(val) => {
                if val.id >= suggestion_id.id {
                    false
                }
                else {
                    true
                }

            }
        }
    }

    pub fn request(&self, suggestion_id: SuggestionId) -> Result<PermissionGranted, Nack> {
        let last_accepted_value = if self.value_map.contains_key(&suggestion_id.key) {
            Some(self.value_map[&suggestion_id.key])
        }
        else {
            None
        };
        let permission_granted = PermissionGranted {
            corresponding_suggestion_id: suggestion_id,
            last_accepted_suggestion_id: self.last_accepted_suggestion_id,
            last_accepted_value: last_accepted_value
        };

        let nack = Nack {
            last_accepted_suggestion_id: self.last_accepted_suggestion_id
        };

        if self.is_newer(suggestion_id) {
            Ok(permission_granted)
        }
        else {
            Err(nack)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*; 
    
    
}