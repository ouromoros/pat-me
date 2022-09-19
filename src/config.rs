use serde_derive::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
}

impl std::default::Default for Config {
    fn default() -> Self {
        Self {  }
    }
}
