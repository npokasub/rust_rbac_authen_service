use crate::database::PgPool;
use crate::models::user::{User, NewUser, UpdateUser};
use crate::repositories::user::UserRepository;
use crate::utilities::error::AppError;
use crate::utilities::encryption::Encryption;
use log::{debug, error, info};
use uuid::Uuid;
use chrono::Utc;

pub struct UserHandler<'a> {
  repo: UserRepository<'a>,
}

impl<'a> UserHandler<'a> {
  pub fn new(pool: &'a PgPool) -> Self {
    debug!("Creating UserHandler");
    Self {
      repo: UserRepository::new(pool),
    }
  }

  pub fn create(&self, username: &str, email: &str, password: &str) -> Result<User, AppError> {
    info!("Creating user: {}", username);
    debug!("Hashing password for user: {}", username);
    let password_hash = Encryption::hash_password(password)?;
    let new_user = NewUser {
      username,
      email,
      password_hash: &password_hash,
    };
    debug!("Calling UserRepository to create user: {}", username);
    let user = self.repo.create(new_user)?;
    info!("User created successfully: {}", username);
    Ok(user)
  }

  pub fn find_by_username(&self, username: &str) -> Result<User, AppError> {
    info!("Looking up user: {}", username);
    debug!("Calling UserRepository to find user: {}", username);
    let user = self.repo.find_by_username(username)?;
    info!("Found user: {}", username);
    Ok(user)
  }

  pub fn find_by_id(&self, id: Uuid) -> Result<User, AppError> {
    info!("Looking up user by ID: {}", id);
    debug!("Calling UserRepository to find user ID: {}", id);
    let user = self.repo.find_by_id(id)?;
    info!("Found user by ID: {}", id);
    Ok(user)
  }

  pub fn update(&self, id: Uuid, username: Option<&str>, email: Option<&str>, password: Option<&str>) -> Result<User, AppError> {
    info!("Updating user: {}", id);
    let password_hash = password.map(|p| {
      debug!("Hashing new password for user: {}", id);
      Encryption::hash_password(p)
    }).transpose()?;
    let update_user = UpdateUser {
      username,
      email,
      password_hash: password_hash.as_deref(),
      updated_at: Utc::now(),
    };
    debug!("Calling UserRepository to update user: {}", id);
    let user = self.repo.update(id, update_user)?;
    info!("User updated successfully: {}", id);
    Ok(user)
  }

  pub fn delete(&self, id: Uuid) -> Result<(), AppError> {
    info!("Deleting user: {}", id);
    debug!("Calling UserRepository to delete user: {}", id);
    self.repo.delete(id)?;
    info!("User deleted successfully: {}", id);
    Ok(())
  }
}