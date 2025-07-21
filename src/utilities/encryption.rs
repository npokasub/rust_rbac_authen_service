use argon2::{
  password_hash::{
    rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString
  },
  Argon2,
};
use crate::utilities::error::AppError;

pub struct Encryption;

impl Encryption {
  pub fn hash_password(password_string: &str) -> Result<String, AppError> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let password_hash = argon2
      .hash_password(password_string.as_bytes(), &salt)
      .map_err(AppError::from)?;
    Ok(password_hash.to_string())
  }

  pub fn verify_password(password_string: &str, password_hashed: &str) -> Result<bool, AppError> {
    let parsed_hash = PasswordHash::new(password_hashed).map_err(AppError::from)?;
    Ok(Argon2::default()
      .verify_password(password_string.as_bytes(), &parsed_hash)
      .is_ok())
  }
}