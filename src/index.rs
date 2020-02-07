use serde_json::json;
use serde_json::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Index {
    fname: String,
    hash: String,
}

impl Index {
    // Create a new entry in index but not write it yet
    pub fn new() -> Self {
        Index {
            fname: String::new(),
            hash: String::new(),
        }
    }

    pub fn read_index() -> Vec<Self> {
	
}
