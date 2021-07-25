use super::errors::FormError;
use super::handlers::NewUserInput;
use super::models::NewUser;
use actix_web::web::Form;
use diesel::Connection;
use serde::Serialize;

/// Trait for validating form
pub trait Validate<T, E = FormError>
where
    T: Serialize,
{
    /// Validate form (ie ensure fields are filled, password is long enough, etc)
    fn validate(self) -> Result<T, E>;
}

impl Validate<NewUserInput> for Form<NewUserInput> {
    fn validate(self) -> Result<NewUserInput, FormError> {
        let p1 = self.password.clone();
        let p2 = self.password_confirm.clone();

        if p1 == p2 {
            if p1.len() >= 8 {
                Ok(self.0)
            } else {
                Err(FormError::PasswordTooShort)
            }
        } else {
            Err(FormError::MismatchPasswords)
        }
    }
}
