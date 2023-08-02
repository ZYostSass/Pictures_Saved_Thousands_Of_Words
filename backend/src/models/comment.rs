use axum::response::{IntoResponse, Response};
use axum::Json;
use derive_more::Display;
use serde_derive::{Deserialize, Serialize};

use crate::make_db_id;
use crate::models::answer::AnswerId;
use crate::models::question::QuestionId;

#[derive(Serialize, Deserialize, Debug, Clone, sqlx::FromRow)]
pub struct Comment {
    pub id: Option<CommentId>,
    pub content: String,
    pub reference: CommentReference,
}

make_db_id!(CommentId);

impl IntoResponse for Comment {
    fn into_response(self) -> Response {
        Json(self).into_response()
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum CommentReference {
    Question(QuestionId),
    Answer(AnswerId),
}
