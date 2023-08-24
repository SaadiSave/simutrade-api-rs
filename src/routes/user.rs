use axum::{
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::Deserialize;
use thiserror::Error;

use crate::{mock::Db, models::User};

// quickly make error type with thiserror
#[derive(Debug, Error)]
pub enum UserError {
    #[error("Username should be at least 3 characters long")]
    UsernameShort,
    #[error("Password should be at least 6 characters long")]
    PasswordShort,
    #[error("Passwords do not match")]
    PasswordsDoNotMatch,
    #[error("Username `{0}` already exists")]
    UserAlreadyExists(String),
}

// implement status code 409 for error
impl IntoResponse for UserError {
    fn into_response(self) -> Response {
        // tuple of (status, body) can be converted to reponse
        (StatusCode::CONFLICT, self.to_string()).into_response()
    }
}

// representation of user request
#[derive(Debug, Deserialize)]
pub struct UserReq {
    name: String,
    password: String,
    confirm_password: String,
}

impl UserReq {
    // validate user
    fn confirm(self) -> Result<User, UserError> {
        if self.name.len() < 3 {
            Err(UserError::UsernameShort)
        } else if self.password.len() < 6 {
            Err(UserError::PasswordShort)
        } else if self.password != self.confirm_password {
            Err(UserError::PasswordsDoNotMatch)
        } else {
            Ok(User {
                name: self.name,
                password: self.password,
            })
        }
    }
}

// route to create user
pub async fn create_user(
    /* Get database */
    State(user_db): State<Db<String, User>>,
    /* Extract details from JSON body (needs Content-Type: application/json) */
    Json(user): Json<UserReq>,
) -> Result<(StatusCode, Json<User>), UserError> {
    // validate and propagate errors
    let user = user.confirm()?;

    if user_db.create(user.name.clone(), user.clone()) {
        // wrap confirmed User in Json to send acknowledgement as JSON
        Ok((StatusCode::CREATED, Json(user)))
    } else {
        // return error if user exists
        Err(UserError::UserAlreadyExists(user.name))
    }
}
