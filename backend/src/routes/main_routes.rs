use axum::response::Response;
use axum::routing::*;
use axum::Router;
use http::StatusCode;
use hyper::Body;
use sqlx::PgPool;

use crate::db::Store;
use crate::handlers::root;
use crate::routes::comment_routes::comment_routes;
use crate::{file_handler, handlers, layers};

pub async fn app(pool: PgPool) -> Router {
    let db = Store::with_pool(pool);

    let (cors_layer, trace_layer) = layers::get_layers();

    let static_router = Router::new()
        .route("/:filename", get(file_handler))
        .with_state(db.clone());

    Router::new()
        // The router matches these FROM TOP TO BOTTOM explicitly!
        //.nest("/static", axum_static::static_router("backend/static").with_state(()))
        .nest("/static", static_router)
        .route("/", get(root))
        .route("/questions", get(handlers::get_questions))
        .route("/question/:question_id", get(handlers::get_question_by_id))
        .route("/question", post(handlers::create_question))
        .route("/question", put(handlers::update_question))
        .route("/question", delete(handlers::delete_question))
        .route("/answer", post(handlers::create_answer))
        .route("/users", post(handlers::register))
        .route("/login", post(handlers::login))
        .route("/protected", get(handlers::protected))
        .route("/*_", get(handle_404))
        .merge(comment_routes())
        .layer(cors_layer)
        .layer(trace_layer)
        .with_state(db)
}

async fn handle_404() -> Response<Body> {
    Response::builder()
        .status(StatusCode::NOT_FOUND)
        .body(Body::from("The requested page could not be found"))
        .unwrap()
}
