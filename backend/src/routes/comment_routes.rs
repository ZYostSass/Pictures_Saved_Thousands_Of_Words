use axum::extract::State;
use axum::routing::{get, post};
use axum::{Json, Router};
pub use axum_macros::debug_handler;

use crate::db::Store;
use crate::error::AppError;
use crate::models::comment::Comment;
use crate::models::page::PagePackage;
use crate::models::question::GetQuestionById;
use crate::routes::main_routes::merged_route;
use crate::AppResult;

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

    #[axum::debug_handler]
    async fn get_comments(
        State(db): State<Store>,
        Json(question): Json<GetQuestionById>,
    ) -> AppResult<PagePackage> {
        let res = db.get_page_for_question(question).await?;

        Ok(res)
    }

    merged_route("/comments", post(create_new_comment)).route("/comments", get(get_comments))
}
