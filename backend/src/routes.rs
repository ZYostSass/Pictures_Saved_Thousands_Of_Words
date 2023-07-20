use axum::response::Response;
use axum::Router;
use axum::routing::*;
use http::StatusCode;
use hyper::Body;

use crate::db::Store;
use crate::error::AppError;
use crate::handlers;

pub async fn get_router() -> Result<Router, AppError> {
    let db = Store::initialize_database_connection().await?;
    let router =Router::new()
        // The router matches these FROM TOP TO BOTTOM explicitly!
        .route("/questions", get(handlers::get_questions))
        // http://localhost:3000/question/1
        .route("/question/:question_id", get(handlers::get_question_by_id))
        .route("/question", post(handlers::create_question))
        .route("/question", put(handlers::update_question))
        .route("/question", delete(handlers::delete_question))
        .route("/*_", get(handle_404))
        .with_state(db);
    Ok(router)
}



async fn handle_404() ->  Response<Body> {
    Response::builder()
        .status(StatusCode::NOT_FOUND)
        .body(Body::from("The requested page could not be found"))
        .unwrap()
}