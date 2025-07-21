use crate::models::user::{User, NewUser, UserResponse};
use crate::repositories::user::UserRepository;
use crate::utilities::error::AppError;
use crate::utilities::encryption::Encryption;
use crate::database::PgPool;

pub struct UserHandler<'a> {
  user_repo: UserRepository<'a>,
}

impl<'a> UserHandler<'a> {
  pub fn new(pool: &'a PgPool) -> Self {
    UserHandler {
      user_repo: UserRepository::new(pool),
    }
  }

  pub fn create(&self, username: &str, email: &str, password: &str) -> Result<UserResponse, AppError> {
    let hashed_password = Encryption::hash_password(password)?;
    let new_user = NewUser {
      username,
      email,
      password_hash: &hashed_password,
    };
    let user = self.user_repo.create(new_user)?;
    Ok(user.into())
  }

  pub fn find_by_id(&self, id: uuid::Uuid) -> Result<UserResponse, AppError> {
    let user = self.user_repo.find_by_id(id)?;
    Ok(user.into())
  }

  pub fn find_by_username(&self, username: &str) -> Result<User, AppError> {
    self.user_repo.find_by_username(username)
  }
}