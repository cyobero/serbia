use serde::{Deserialize, Serialize};

#[derive(Fail, Debug, Serialize, Deserialize)]
pub enum FormError {
    #[fail(display = "Passwords do not match.")]
    MismatchPasswords,

    #[fail(display = "Password must be at least 8 characters long.")]
    PasswordTooShort,

    #[fail(display = "Form cannot be empty.")]
    EmptyForm,
}

#[derive(Fail, Debug, Serialize, Deserialize)]
pub enum EmptyFieldError {
    #[fail(display = "Password field cannot be empty.")]
    EmptyPassword,

    #[fail(display = "Username field cannot be empty.")]
    EmptyUserName,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum AuthError {
    InvalidPassword,
    UserNotFound,
    UserAlreadyExists,
}
