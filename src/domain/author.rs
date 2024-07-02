use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Author {
    pub firstname:String,
    pub lastname:String,
}