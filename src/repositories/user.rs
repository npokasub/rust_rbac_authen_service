use diesel::prelude::*;
use uuid::Uuid;
use crate::schema::users;
use crate::models::user::{User, NewUser, UpdateUser};
use crate::database::PgPool;
use crate::utilities::error::AppError;
use log::{debug, error, info};

pub struct UserRepository<'a> {
  conn: &'a PgPool,
}

impl<'a> UserRepository<'a> {
  pub fn new(conn: &'a PgPool) -> Self {
    debug!("Creating UserRepository");
    Self { conn }
  }

  pub fn create(&self, new_user: NewUser) -> Result<User, AppError> {
    info!("Creating user in repository: {}", new_user.username);
    let mut conn = self.conn.get().map_err(|e| {
      error!("Failed to get database connection: {}", e);
      AppError::ConnectionError(format!("Failed to get database connection: {}", e))
    })?;
    debug!("Inserting user into database: {}", new_user.username);
    let user: User = conn.transaction(|conn| {
      diesel::insert_into(users::table)
        .values(&new_user)
        .get_result(conn)
        .map_err(|e| {
          error!("Failed to create user {}: {:?}", new_user.username, e);
          AppError::from(e)
        })
    })?;
    info!("User created successfully in repository: {}", user.username);
    Ok(user)
  }

  pub fn find_by_id(&self, id: Uuid) -> Result<User, AppError> {
    info!("Looking up user by ID in repository: {}", id);
    let mut conn = self.conn.get().map_err(|e| {
      error!("Failed to get database connection: {}", e);
      AppError::ConnectionError(format!("Failed to get database connection: {}", e))
    })?;
    debug!("Querying database for user ID: {}", id);
    let user = users::table
      .find(id)
      .first(&mut conn)
      .map_err(|e| {
        error!("Failed to find user with ID {}: {:?}", id, e);
        AppError::from(e)
      })?;
    info!("Found user by ID in repository: {}", id);
    Ok(user)
  }

  pub fn find_by_username(&self, username: &str) -> Result<User, AppError> {
    info!("Looking up user by username in repository: {}", username);
    let mut conn = self.conn.get().map_err(|e| {
      error!("Failed to get database connection: {}", e);
      AppError::ConnectionError(format!("Failed to get database connection: {}", e))
    })?;
    debug!("Querying database for user: {}", username);
    let user = users::table
      .filter(users::username.eq(username))
      .first(&mut conn)
      .map_err(|e| {
        error!("Failed to find user with username {}: {:?}", username, e);
        AppError::from(e)
      })?;
    info!("Found user by username in repository: {}", username);
    Ok(user)
  }

  pub fn update(&self, id: Uuid, update_user: UpdateUser) -> Result<User, AppError> {
    info!("Updating user in repository: {}", id);
    let mut conn = self.conn.get().map_err(|e| {
      error!("Failed to get database connection: {}", e);
      AppError::ConnectionError(format!("Failed to get database connection: {}", e))
    })?;
    debug!("Updating user in database: {}", id);
    let user = conn.transaction(|conn| {
      diesel::update(users::table.find(id))
        .set(&update_user)
        .get_result(conn)
        .map_err(|e| {
          error!("Failed to update user with ID {}: {:?}", id, e);
          AppError::from(e)
        })
    })?;
    info!("User updated successfully in repository: {}", id);
    Ok(user)
  }

  pub fn delete(&self, id: Uuid) -> Result<(), AppError> {
    info!("Deleting user in repository: {}", id);
    let mut conn = self.conn.get().map_err(|e| {
      error!("Failed to get database connection: {}", e);
      AppError::ConnectionError(format!("Failed to get database connection: {}", e))
    })?;
    debug!("Deleting user from database: {}", id);
    let affected = conn.transaction(|conn| {
      diesel::delete(users::table.find(id))
          .execute(conn)
          .map_err(|e| {
            error!("Failed to delete user with ID {}: {:?}", id, e);
            AppError::from(e)
          })
    })?;
    if affected == 0 {
      error!("User with ID {} not found for deletion", id);
      return Err(AppError::NotFound(format!("User with ID {} not found", id)));
    }
    info!("User deleted successfully in repository: {}", id);
    Ok(())
  }
}