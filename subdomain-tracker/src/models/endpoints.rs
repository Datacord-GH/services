use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Endpoint {
    pub description: String,
    pub url: String,
}

impl Endpoint {
    pub fn get_clean_url(&self) -> String {
        return self.url.replace("https://", "");
    }
}
