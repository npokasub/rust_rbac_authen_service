use diesel::prelude::*;
use uuid::Uuid;
use crate::schema::permissions;
use crate::models::permission::{Permission, NewPermission, UpdatePermission};
use crate::database::PgPool;
use crate::utilities::error::AppError;
use log::{debug, error, info};

pub struct PermissionRepository<'a> {
  conn: &'a PgPool,
}

impl<'a> PermissionRepository<'a> {
  pub fn new(conn: &'a PgPool) -> Self {
    debug!("Creating PermissionRepository");
    Self { conn }
  }

  pub fn create(&self, new_permission: NewPermission) -> Result<Permission, AppError> {
    info!("Creating permission in repository: {}", new_permission.name);
    let mut conn = self.conn.get().map_err(|e| {
      error!("Failed to get database connection: {}", e);
      AppError::ConnectionError(format!("Failed to get database connection: {}", e))
    })?;
    debug!("Inserting permission into database: {}", new_permission.name);
    let permission: Permission = conn.transaction(|conn| {
      diesel::insert_into(permissions::table)
        .values(&new_permission)
        .get_result(conn)
        .map_err(|e| {
          error!("Failed to create permission {}: {:?}", new_permission.name, e);
          AppError::from(e)
        })
    })?;
    info!("Permission created successfully in repository: {}", permission.name);
    Ok(permission)
  }

  pub fn find_by_id(&self, id: Uuid) -> Result<Permission, AppError> {
    info!("Looking up permission by ID in repository: {}", id);
    let mut conn = self.conn.get().map_err(|e| {
      error!("Failed to get database connection: {}", e);
      AppError::ConnectionError(format!("Failed to get database connection: {}", e))
    })?;
    debug!("Querying database for permission ID: {}", id);
    let permission = permissions::table
      .find(id)
      .first(&mut conn)
      .map_err(|e| {
        error!("Failed to find permission with ID {}: {:?}", id, e);
        AppError::from(e)
      })?;
    info!("Found permission by ID in repository: {}", id);
    Ok(permission)
  }

  pub fn find_by_name(&self, name: &str) -> Result<Permission, AppError> {
    info!("Looking up permission by name in repository: {}", name);
    let mut conn = self.conn.get().map_err(|e| {
      error!("Failed to get database connection: {}", e);
      AppError::ConnectionError(format!("Failed to get database connection: {}", e))
    })?;
    debug!("Querying database for permission: {}", name);
    let permission = permissions::table
      .filter(permissions::name.eq(name))
      .first(&mut conn)
      .map_err(|e| {
        error!("Failed to find permission with name {}: {:?}", name, e);
        AppError::from(e)
      })?;
    info!("Found permission by name in repository: {}", name);
    Ok(permission)
  }

  pub fn update(&self, id: Uuid, update_permission: UpdatePermission) -> Result<Permission, AppError> {
    info!("Updating permission in repository: {}", id);
    let mut conn = self.conn.get().map_err(|e| {
      error!("Failed to get database connection: {}", e);
      AppError::ConnectionError(format!("Failed to get database connection: {}", e))
    })?;
    debug!("Updating permission in database: {}", id);
    let permission = conn.transaction(|conn| {
      diesel::update(permissions::table.find(id))
        .set(&update_permission)
        .get_result(conn)
        .map_err(|e| {
          error!("Failed to update permission with ID {}: {:?}", id, e);
          AppError::from(e)
        })
    })?;
    info!("Permission updated successfully in repository: {}", id);
    Ok(permission)
  }

  pub fn delete(&self, id: Uuid) -> Result<(), AppError> {
    info!("Deleting permission in repository: {}", id);
    let mut conn = self.conn.get().map_err(|e| {
      error!("Failed to get database connection: {}", e);
      AppError::ConnectionError(format!("Failed to get database connection: {}", e))
    })?;
    debug!("Deleting permission from database: {}", id);
    let affected = conn.transaction(|conn| {
      diesel::delete(permissions::table.find(id))
        .execute(conn)
        .map_err(|e| {
          error!("Failed to delete permission with ID {}: {:?}", id, e);
          AppError::from(e)
        })
    })?;
    if affected == 0 {
      error!("Permission with ID {} not found for deletion", id);
      return Err(AppError::NotFound(format!("Permission with ID {} not found", id)));
    }
    info!("Permission deleted successfully in repository: {}", id);
    Ok(())
  }
}