use crate::core::{Suggestion, Accept, Nack, PermissionGranted};

pub struct Cache {
    value: Option<u32>,
    pub cache_id: u32,
    last_accepted_suggestion: Option<Suggestion>,
    is_online: bool
}

impl Cache {

    pub fn new(id: u32, is_online: bool) -> Cache {
        Cache {
            value: None,
            cache_id: id,
            last_accepted_suggestion: None,
            is_online: is_online
        }
    }

    pub fn get(&self) -> Option<u32> {
        self.value
    }

    pub fn set_online_status(&mut self, status: bool) {
        self.is_online = status;
    }

    pub fn generate_suggestion(&self, proposed_value: u32) -> Option<Suggestion> {
        let last_suggestion_id = match self.last_accepted_suggestion {
            None => 0,
            Some(val) => val.id
        };
        let suggestion = Suggestion {
            id: last_suggestion_id + 1,
            cache_id: self.cache_id,
            value: proposed_value
        };
        if self.is_online {

            println!("Cache {} created suggestion with id {}.", self.cache_id, suggestion.id);
            Some(suggestion)
        }
        else {
            println!("Cache {} was asked to create a suggestion, but it was offline.", self.cache_id);
            None
        }
    }

    pub fn suggest(&mut self, suggestion: Suggestion) -> Result<Accept, Nack> {
        if !self.is_online {
            println!("Cache {} rejects suggestion {}_{} due to being offline", self.cache_id, suggestion.id, suggestion.cache_id);
            return Err(Nack {
                suggestion: suggestion
            });
        }
        let accept = Accept {
            suggestion: suggestion
        };
        match self.get_nack_if_suggestion_is_old(suggestion) {
            None => {
                println!("Cache {} has accepted suggestion id {}_{}.", self.cache_id, suggestion.id, suggestion.cache_id);
                self.last_accepted_suggestion = Some(suggestion);
                self.value = Some(suggestion.value);
                Ok(accept)
            }
            Some(nack) => Err(nack)
        }
    }

    pub fn request(&self, suggestion: Suggestion) -> Result<PermissionGranted, Nack> {
        if !self.is_online {
            println!("Cache {} rejects request {}_{} due to being offline", self.cache_id, suggestion.id, suggestion.cache_id);
            return Err(Nack {
                suggestion: suggestion
            });
        }
        let last_accepted_value = self.get();
        let permission_granted = PermissionGranted {
            corresponding_suggestion: suggestion,
            last_accepted_suggestion: self.last_accepted_suggestion,
            last_accepted_value: last_accepted_value
        };

        match self.get_nack_if_suggestion_is_old(suggestion) {
            None => {
                println!("Cache {} grants permission to accept suggestion id {}_{}", self.cache_id, suggestion.id, suggestion.cache_id);
                Ok(permission_granted)
            },
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
                    
                    println!("Cache {} rejects this suggestion, last accepted id is {}_{}", self.cache_id, val.id, val.cache_id);
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