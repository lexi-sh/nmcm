
#[derive(Debug, Copy, Clone)]
pub struct Suggestion {
    pub id: u32,
    pub cache_id: u32,
    pub key: u32,
    pub value: u32
}

pub struct SuggestionResponse {
    pub suggestion: Suggestion
}


pub type Nack = SuggestionResponse;
pub type Accept = SuggestionResponse;

pub struct PermissionGranted {
    pub corresponding_suggestion: Suggestion,
    pub last_accepted_suggestion: Option<Suggestion>,
    pub last_accepted_value: Option<u32>
}
