use derive_more::Display;
use serde_derive::{Deserialize, Serialize};

use crate::make_db_id;

#[derive(Clone, Debug, Display, Serialize, Deserialize, sqlx::FromRow)]
#[display(
    fmt = "id: {}, title: {}, content: {}, tags: {:?}",
    id,
    title,
    content,
    tags
)]
pub struct Question {
    pub id: QuestionId,
    pub title: String,
    pub content: String,
    pub tags: Option<Vec<String>>,
}

impl Question {
    #[allow(dead_code)]
    pub fn new(id: QuestionId, title: String, content: String, tags: Option<Vec<String>>) -> Self {
        Question {
            id,
            title,
            content,
            tags,
        }
    }
}

// Generate pkey bindings
make_db_id!(QuestionId);

// Clients use this to create new requests
#[derive(Debug, Serialize, Deserialize)]
pub struct CreateQuestion {
    pub title: String,
    pub content: String,
    pub tags: Option<Vec<String>>,
}

#[derive(Deserialize)]
pub struct GetQuestionById {
    pub question_id: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateQuestion {
    pub id: QuestionId,
    pub title: String,
    pub content: String,
    pub tags: Option<Vec<String>>,
}
