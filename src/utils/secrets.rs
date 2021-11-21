use crate::service::errors::ServiceError;
use argon2::{self, Config};

lazy_static::lazy_static! {
  pub static ref SECRET_KEY: String = std::env::var("SECRET_KEY").unwrap_or_else(|_| "0123".repeat(8));
}

/// we will pull this from some secret store like vault
const SALT: &'static [u8] = b"supersecuresalt";

/// WARNING DO THIS RIGHT 
pub fn hash_password(password: &str) -> Result<String, ServiceError> {
  let config = Config {secret: SECRET_KEY.as_bytes(), ..Default::default()};

  argon2::hash_encoded(password.as_bytes(), &SALT, &config).map_err(|err| {
    dbg!(err);
    ServiceError::InternalServerError
  })
}

/// use some salt, and verify the password is valid
pub fn verify(hash: &str, password: &str) -> Result<bool, ServiceError> {
  argon2::verify_encoded_ext(hash, password.as_bytes(), SECRET_KEY.as_bytes(), &[]).map_err(
    |err| {
      dbg!(err);
      ServiceError::Unauthorized
    },
  )
}
