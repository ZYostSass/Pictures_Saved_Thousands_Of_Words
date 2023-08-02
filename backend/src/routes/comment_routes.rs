use axum::extract::State;
use axum::routing::post;
use axum::{Json, Router};
use serde_derive::{Deserialize, Serialize};
use tracing::info;

use crate::db::Store;
use crate::error::AppError;
use crate::models::answer::AnswerId;
use crate::models::question::QuestionId;

use crate::routes::main_routes::merged_route;
use crate::AppResult;

pub fn comment_routes() -> Router<Store> {
    merged_route("/comments", post(create_new_comment))
}

pub async fn create_new_comment(
    State(db): State<Store>,
    Json(comment): Json<AddComment>,
) -> AppResult<()> {
    //if comment.content.is_empty() {
    //    return Err(AppError::MissingContent);
    //}

    info!("Some stuff");

    Ok(())
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AddComment {
    pub content: String,
    pub reference: CommentReference,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum CommentReference {
    Question(QuestionId),
    Answer(AnswerId),
}
