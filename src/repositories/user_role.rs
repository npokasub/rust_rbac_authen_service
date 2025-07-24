use diesel::prelude::*;
use uuid::Uuid;
use crate::schema::user_roles;
use crate::models::user_role::{UserRole, NewUserRole};
use crate::database::PgPool;
use crate::utilities::error::AppError;
use log::{debug, error, info};

pub struct UserRoleRepository<'a> {
  conn: &'a PgPool,
}

impl<'a> UserRoleRepository<'a> {
  pub fn new(conn: &'a PgPool) -> Self {
    debug!("Creating UserRoleRepository");
    Self { conn }
  }

  pub fn create(&self, user_id: Uuid, role_id: Uuid) -> Result<UserRole, AppError> {
    info!("Creating user_role in repository: user_id={}, role_id={}", user_id, role_id);
    let mut conn = self.conn.get().map_err(|e| {
      error!("Failed to get database connection: {}", e);
      AppError::ConnectionError(format!("Failed to get database connection: {}", e))
    })?;
    let new_user_role = NewUserRole {
      user_id,
      role_id,
    };
    debug!("Inserting user_role into database: user_id={}, role_id={}", user_id, role_id);
    let user_role: UserRole = conn.transaction(|conn| {
      diesel::insert_into(user_roles::table)
          .values(&new_user_role)
          .get_result(conn)
          .map_err(|e| {
            error!("Failed to create user_role for user_id={} and role_id={}: {:?}", user_id, role_id, e);
            AppError::from(e)
          })
    })?;
    info!("UserRole created successfully in repository: user_id={}, role_id={}", user_id, role_id);
    Ok(user_role)
  }

  pub fn find_by_ids(&self, user_id: Uuid, role_id: Uuid) -> Result<UserRole, AppError> {
    info!("Looking up user_role by user_id={} and role_id={}", user_id, role_id);
    let mut conn = self.conn.get().map_err(|e| {
      error!("Failed to get database connection: {}", e);
      AppError::ConnectionError(format!("Failed to get database connection: {}", e))
    })?;
    debug!("Querying database for user_role: user_id={}, role_id={}", user_id, role_id);
    let user_role = user_roles::table
      .filter(user_roles::user_id.eq(user_id))
      .filter(user_roles::role_id.eq(role_id))
      .first(&mut conn)
      .map_err(|e| {
        error!("Failed to find user_role with user_id={} and role_id={}: {:?}", user_id, role_id, e);
        AppError::from(e)
      })?;
    info!("Found user_role: user_id={}, role_id={}", user_id, role_id);
    Ok(user_role)
  }

  pub fn find_by_user_id(&self, user_id: Uuid) -> Result<Vec<UserRole>, AppError> {
    info!("Looking up user_roles by user_id={}", user_id);
    let mut conn = self.conn.get().map_err(|e| {
      error!("Failed to get database connection: {}", e);
      AppError::ConnectionError(format!("Failed to get database connection: {}", e))
    })?;
    debug!("Querying database for user_roles with user_id={}", user_id);
    let user_roles = user_roles::table
      .filter(user_roles::user_id.eq(user_id))
      .load::<UserRole>(&mut conn)
      .map_err(|e| {
        error!("Failed to retrieve user_roles for user_id={}: {:?}", user_id, e);
        AppError::from(e)
      })?;
    info!("Found {} user_roles for user_id={}", user_roles.len(), user_id);
    Ok(user_roles)
  }

  pub fn delete(&self, user_id: Uuid, role_id: Uuid) -> Result<(), AppError> {
    info!("Deleting user_role in repository: user_id={}, role_id={}", user_id, role_id);
    let mut conn = self.conn.get().map_err(|e| {
      error!("Failed to get database connection: {}", e);
      AppError::ConnectionError(format!("Failed to get database connection: {}", e))
    })?;
    debug!("Deleting user_role from database: user_id={}, role_id={}", user_id, role_id);
    let affected = conn.transaction(|conn| {
      diesel::delete(
        user_roles::table
            .filter(user_roles::user_id.eq(user_id))
            .filter(user_roles::role_id.eq(role_id))
      )
      .execute(conn)
      .map_err(|e| {
        error!("Failed to delete user_role with user_id={} and role_id={}: {:?}", user_id, role_id, e);
        AppError::from(e)
      })
    })?;
    if affected == 0 {
      error!("UserRole with user_id={} and role_id={} not found for deletion", user_id, role_id);
      return Err(AppError::NotFound(format!("UserRole with user_id={} and role_id={} not found", user_id, role_id)));
    }
    info!("UserRole deleted successfully: user_id={}, role_id={}", user_id, role_id);
    Ok(())
  }
}