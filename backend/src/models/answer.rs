use crate::make_db_id;
use crate::models::question::QuestionId;
use derive_more::Display;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Answer {
    pub id: AnswerId,
    pub content: String,
    pub question_id: QuestionId,
}

make_db_id!(AnswerId);

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateAnswer {
    pub content: String,
    pub question_id: i32,
}
