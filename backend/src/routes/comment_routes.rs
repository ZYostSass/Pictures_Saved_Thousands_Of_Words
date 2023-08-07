use crate::db::Store;
use crate::error::AppError;
use axum::extract::State;
use axum::routing::{get, post, MethodRouter};
use axum::{Json, Router};

use crate::models::comment::Comment;
use crate::models::page::PagePackage;
use crate::models::question::GetQuestionById;

pub fn comment_routes() -> Router<Store> {
    async fn create_new_comment(
        State(db): State<Store>,
        Json(comment): Json<Comment>,
    ) -> Result<Comment, AppError> {
        let comment = db.create_comment(comment).await?;
        Ok(comment)
    }

    async fn get_comments(
        State(db): State<Store>,
        Json(question): Json<GetQuestionById>,
    ) -> Result<PagePackage, AppError> {
        let questions = db.get_page_for_question(question).await?;

        Ok(questions)
    }

    merged_route("/comments", post(create_new_comment)).route("/comments", get(get_comments))
}

pub fn merged_route<T>(path: &str, method_router: MethodRouter<T>) -> Router<T>
where
    T: Clone + Send + Sync + 'static,
{
    Router::new().route(path, method_router)
}
