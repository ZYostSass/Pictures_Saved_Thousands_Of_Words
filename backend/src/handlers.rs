use axum::extract::{Path, Query, State};
use axum::response::Html;
use axum::Json;
use jsonwebtoken::Header;
use serde_json::{json, Value};
use tera::Context;
use tracing::error;

use crate::answer::{Answer, CreateAnswer};
use crate::db::Store;
use crate::error::AppError;
use crate::get_timestamp_after_8_hours;
use crate::question::{CreateQuestion, GetQuestionById, Question, QuestionId, UpdateQuestion};
use crate::template::TEMPLATES;
use crate::user::{Claims, User, UserSignup, KEYS};

#[allow(dead_code)]
pub async fn root() -> Html<String> {
    let mut context = Context::new();
    context.insert("name", "Casey");
    let rendered = TEMPLATES
        .render("index.html", &context)
        .unwrap_or_else(|err| {
            error!("Template rendering error: {}", err);
            panic!()
        });
    Html(rendered)
}

// CRUD create - read - update - delete
pub async fn get_questions(
    State(mut am_database): State<Store>,
) -> Result<Json<Vec<Question>>, AppError> {
    let all_questions = am_database.get_all_questions().await?;

    Ok(Json(all_questions))
}

pub async fn get_question_by_id(
    State(mut am_database): State<Store>,
    Path(query): Path<i32>, // localhost:3000/question/5
) -> Result<Json<Question>, AppError> {
    let question = am_database.get_question_by_id(QuestionId(query)).await?;
    Ok(Json(question))
}

pub async fn create_question(
    State(mut am_database): State<Store>,
    Json(question): Json<CreateQuestion>,
) -> Result<Json<Question>, AppError> {
    let question = am_database
        .add_question(question.title, question.content, question.tags)
        .await?;

    Ok(Json(question))
}

pub async fn update_question(
    State(mut am_database): State<Store>,
    Json(question): Json<UpdateQuestion>,
) -> Result<Json<Question>, AppError> {
    let updated_question = am_database.update_question(question).await?;
    Ok(Json(updated_question))
}

pub async fn delete_question(
    State(mut am_database): State<Store>,
    Query(query): Query<GetQuestionById>,
) -> Result<(), AppError> {
    am_database.delete_question(query.question_id).await?;

    Ok(())
}

pub async fn create_answer(
    State(mut am_database): State<Store>,
    Json(answer): Json<CreateAnswer>,
) -> Result<Json<Answer>, AppError> {
    dbg!("GOT CREATE ANSWER:");
    dbg!(&answer);
    let new_answer = am_database
        .add_answer(answer.content, answer.question_id)
        .await?;
    Ok(Json(new_answer))
}

pub async fn register(
    State(mut database): State<Store>,
    Json(credentials): Json<UserSignup>,
) -> Result<Json<Value>, AppError> {
    // We should also check to validate other things at some point like email address being in right format

    if credentials.email.is_empty() || credentials.password.is_empty() {
        return Err(AppError::MissingCredentials);
    }

    if credentials.password != credentials.confirm_password {
        return Err(AppError::MissingCredentials);
    }

    // Check to see if there is already a user in the database with the given email address
    let existing_user = database.get_user(&credentials.email).await;

    if let Ok(_) = existing_user {
        return Err(AppError::UserAlreadyExists);
    }

    let new_user = database.create_user(credentials).await?;
    Ok(new_user)
}

pub async fn login(
    State(mut database): State<Store>,
    Json(creds): Json<User>,
) -> Result<Json<Value>, AppError> {
    if creds.email.is_empty() || creds.password.is_empty() {
        return Err(AppError::MissingCredentials);
    }

    let existing_user = database.get_user(&creds.email).await?;

    if existing_user.password != creds.password {
        Err(AppError::MissingCredentials)
    } else {
        // at this point we've authenticated the user's identity
        // create JWT to return
        let claims = Claims {
            id: 0,
            email: creds.email.to_owned(),
            exp: get_timestamp_after_8_hours(),
        };

        let token = jsonwebtoken::encode(&Header::default(), &claims, &KEYS.encoding)
            .map_err(|_| AppError::MissingCredentials)?;
        Ok(Json(json!({ "access_token" : token, "type": "Bearer"})))
    }
}

pub async fn protected(claims: Claims) -> Result<String, AppError> {
    Ok(format!(
        "Welcome to the PROTECTED area :) \n Your claim data is: {}",
        claims
    ))
}
