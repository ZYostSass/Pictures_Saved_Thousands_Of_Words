use axum::extract::State;
use axum::routing::post;
use axum::{Json, Router};
use serde_derive::{Deserialize, Serialize};
use tracing::info;

use crate::db::Store;
use crate::error::AppError;
use crate::models::answer::AnswerId;
use crate::models::question::QuestionId;

use crate::models::comment::Comment;
use crate::routes::main_routes::merged_route;
use crate::AppResult;
pub use axum_macros::debug_handler;

//#[axum::debug_handler]
pub fn comment_routes() -> Router<Store> {
    async fn create_new_comment(
        State(db): State<Store>,
        Json(comment): Json<Comment>,
    ) -> AppResult<Comment> {
        if comment.content.is_empty() {
            return Err(AppError::MissingContent);
        }

        let comment = db.create_comment(comment).await?;

        Ok(comment)
    }

    merged_route("/comments", post(create_new_comment))
}
