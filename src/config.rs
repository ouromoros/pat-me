use serde_derive::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub default_method: String,
    pub default_title: String,
    pub default_msg: String,

    pub email_config: Option<EmailConfig>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EmailConfig {
    pub server: String,
    pub username: String,
    pub password: String,
    pub to: Option<String>,
}

impl std::default::Default for Config {
    fn default() -> Self {
        Self { 
            default_method: "desktop".to_string(),
            default_title: "[Empty Title]".to_string(),
            default_msg: "Oops, empty message body.".to_string(),

            email_config: None
        }
    }
}
