use serde::{Deserialize, Serialize};

#[derive(Fail, Debug, Serialize, Deserialize)]
pub enum FormError {
    #[fail(display = "Passwords do not match.")]
    MismatchPasswords,

    #[fail(display = "This field is too short.")]
    FieldTooShort(String),

    #[fail(display = "Field cannot be empty.")]
    EmptyField(String),
}

#[derive(Debug, Deserialize, Serialize)]
pub enum AuthError {
    InvalidPassword,
    UserNotFound,
    UserAlreadyExists,
}
