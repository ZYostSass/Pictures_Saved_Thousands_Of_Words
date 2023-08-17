use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Apod {
    pub id: ApodId,
    pub user_id: i32,
    pub date: String,
    pub title: String,
    pub explanation: String,
    pub media_type: String,
    pub url: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct ApodId(pub i32);

impl From<i32> for ApodId {
    fn from(value: i32) -> Self {
        Self(value)
    }
}

impl Apod {
    
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateApod {
    pub user_id: i32,
    pub date: String,
    pub title: String,
    pub explanation: String,
    pub media_type: String,
    pub url: String,
}

#[derive(Debug, Deserialize)]
pub struct ApodDisplayData{
    pub title: String,
    pub explanation: String,
    pub url: String,
}