use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Content {
    pub id:i32,
    pub title:String,
    pub slug:String,
    pub thumbnail:String,
    pub image:String,
    pub description:String,
    pub body:String,
}