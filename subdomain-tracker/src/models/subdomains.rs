use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubdomainDB {
    pub domain: String,
    pub subdomain: String,
    pub date: String,
}
