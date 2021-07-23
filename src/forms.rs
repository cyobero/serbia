use super::errors::FormError;
use diesel::Connection;
use serde::Serialize;

/// Trait for validating form
pub trait Validate<T, C, E = FormError>
where
    T: Serialize,
    C: Connection,
{
    type Output;

    /// Validate form (ie ensure fields are filled, password is long enough, etc)
    fn validate(&self) -> Result<Self::Output, E>;

    /// Boolean that returns `true` if `validate()` has already been called and returned `Ok`
    fn is_valid() -> bool;
}
