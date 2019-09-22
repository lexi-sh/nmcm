use crate::core::{Suggestion, Accept, Nack, PermissionGranted};

pub struct Cache {
    value: Option<u32>,
    cache_id: u32,
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
            Some(suggestion)
        }
        else {
            None
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

    pub fn request(&self, suggestion: Suggestion) -> Result<PermissionGranted, Nack> {
        let last_accepted_value = self.get();
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