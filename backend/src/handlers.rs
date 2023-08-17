use argon2::Config;
use axum::extract::{Path, Query, State};
use axum::handler::Handler;
use axum::response::{Html, Response};
use axum::{Form, Json};
use http::header::{LOCATION, SET_COOKIE};
use http::{HeaderValue, StatusCode};
use hyper::Body;
use jsonwebtoken::Header;
use serde_json::Value;
use tera::Context;
use tracing::error;

use crate::db::Store;
use crate::error::AppError;
use crate::get_timestamp_after_8_hours;
use crate::models::answer::{Answer, CreateAnswer};
use crate::models::question::{
    CreateQuestion, GetQuestionById, Question, QuestionId, UpdateQuestion,
};
use crate::models::user::{Claims, OptionalClaims, User, UserSignup, KEYS};

use crate::models::{Apod, CreateApod, ApodDisplayData};
use crate::template::TEMPLATES;

#[allow(dead_code)]
pub async fn root(
    State(am_database): State<Store>,
    OptionalClaims(claims): OptionalClaims,
) -> Result<Html<String>, AppError> {

    //kidnapped casey's code to redirect logged in users to most recent POD!
    //uses my api key; hopefully thats not a problem...
    let mut context = Context::new();


    let template_name = if let Some(claims_data) = claims {
        // helper functions, so to speak. gets current POD.
        let apod_api_key = "HbP7U12I4K6CKbozeINP0PogXXL0fbiabLZ7jVjf"; // Replace with your NASA API key
        let apod_response = get_apod(apod_api_key).await; // Use the question mark operator to handle errors
    
        let apod_data = apod_response.map_err(|_err| AppError::InternalServerError)?;
    
        context.insert("apod_url", &apod_data.url);
        context.insert("apod_title", &apod_data.title);
        context.insert("apod_explanation", &apod_data.explanation);

        "apod_page.html" // Use the new template when logged in
    } else {
        // Handle the case where the user isn't logged in
        error!("is_logged_in is FALSE now");
        context.insert("is_logged_in", &false);
        "index.html" // Use the original template when not logged in
    };

    let rendered = TEMPLATES
        .render(template_name, &context)
        .unwrap_or_else(|err| {
            error!("Template rendering error: {}", err);
            panic!()
        });
    Ok(Html(rendered))
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
    let new_answer = am_database
        .add_answer(answer.content, answer.question_id)
        .await?;
    Ok(Json(new_answer))
}

pub async fn register(
    State(database): State<Store>,
    Json(mut credentials): Json<UserSignup>,
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

    // Here we're assured that our credentials are valid and the user doesn't already exist
    // hash their password
    let hash_config = Config::default();
    let salt = std::env::var("SALT").expect("Missing SALT");
    let hashed_password = match argon2::hash_encoded(
        credentials.password.as_bytes(),
        // If you'd like unique salts per-user, simply pass &[] and argon will generate them for you
        salt.as_bytes(),
        &hash_config,
    ) {
        Ok(result) => result,
        Err(_) => {
            return Err(AppError::Any(anyhow::anyhow!("Password hashing failed")));
        }
    };

    credentials.password = hashed_password;

    let new_user = database.create_user(credentials).await?;
    Ok(new_user)
}

pub async fn login(
    State(database): State<Store>,
    Form(creds): Form<User>,
) -> Result<Response<Body>, AppError> {
    if creds.email.is_empty() || creds.password.is_empty() {
        return Err(AppError::MissingCredentials);
    }

    let existing_user = database.get_user(&creds.email).await?;

    let is_password_correct =
        match argon2::verify_encoded(&*existing_user.password, creds.password.as_bytes()) {
            Ok(result) => result,
            Err(_) => {
                return Err(AppError::InternalServerError);
            }
        };

    if !is_password_correct {
        return Err(AppError::InvalidPassword);
    }

    // at this point we've authenticated the user's identity
    // create JWT to return
    let claims = Claims {
        id: 0,
        email: creds.email.to_owned(),
        exp: get_timestamp_after_8_hours(),
    };

    let token = jsonwebtoken::encode(&Header::default(), &claims, &KEYS.encoding)
        .map_err(|_| AppError::MissingCredentials)?;

    let cookie = cookie::Cookie::build("jwt", token).http_only(true).finish();

    let mut response = Response::builder()
        .status(StatusCode::FOUND)
        .body(Body::empty())
        .unwrap();

    response
        .headers_mut()
        .insert(LOCATION, HeaderValue::from_static("/"));
    response.headers_mut().insert(
        SET_COOKIE,
        HeaderValue::from_str(&cookie.to_string()).unwrap(),
    );

    Ok(response)
}

pub async fn protected(claims: Claims) -> Result<String, AppError> {
    Ok(format!(
        "Welcome to the PROTECTED area :) \n Your claim data is: {}",
        claims
    ))
}

pub async fn create_apod(
    State(mut am_database): State<Store>,
    Json(apod): Json<CreateApod>,
) -> Result<Json<Apod>, AppError> {
    let new_apod = am_database
        .add_apod(apod.user_id, apod.date, apod.title, apod.explanation, apod.media_type, apod.url)
        .await?;
    
    Ok(Json(new_apod))
}

pub async fn get_apods(
    State(mut am_database): State<Store>,
) -> Result<Json<Vec<Apod>>, AppError> {
    let all_apods = am_database.get_all_apods().await?;

    Ok(Json(all_apods))
}

async fn get_apod(api_key: &str) -> Result<ApodDisplayData, reqwest::Error> {
    let url = format!("https://api.nasa.gov/planetary/apod?api_key={}", api_key);
    let response = reqwest::get(&url).await?.json().await?;
    Ok(response)
}

pub async fn save_apod_to_account(
    State(mut am_database): State<Store>,
    Json(apod): Json<CreateApod>,
    OptionalClaims(claims): OptionalClaims,
) -> Result<Json<Apod>, AppError> {
    if let Some(user_id) = claims.map(|claims| claims.id) {
        // If the user is logged in, save the APOD to their account
        let new_apod = am_database
            .add_apod(user_id, apod.date, apod.title, apod.explanation, apod.media_type, apod.url)
            .await?;

        Ok(Json(new_apod))
    } else {
        // If the user is not logged in, return an error
        Err(AppError::InternalServerError)
    }
}

pub async fn apod_page(
    OptionalClaims(_claims): OptionalClaims,
) -> Result<Html<String>, AppError> {
    let apod_api_key = "HbP7U12I4K6CKbozeINP0PogXXL0fbiabLZ7jVjf"; // Replace with your NASA API key
    let apod_response = get_apod(apod_api_key).await; // Use the question mark operator to handle errors

    let apod_data = apod_response.map_err(|_err| AppError::InternalServerError)?;

    let mut context = Context::new();
    context.insert("apod_url", &apod_data.url);
    context.insert("apod_title", &apod_data.title);
    context.insert("apod_explanation", &apod_data.explanation);

    let rendered = TEMPLATES
        .render("apod_page.html", &context)
        .map_err(|err| {
            error!("Template rendering error: {}", err);
            AppError::InternalServerError // Convert rendering error to your AppError
        })?;

    Ok(Html(rendered))
}