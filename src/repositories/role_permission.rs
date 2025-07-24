use diesel::prelude::*;
use uuid::Uuid;
use crate::schema::role_permissions;
use crate::models::role_permission::{RolePermission, NewRolePermission};
use crate::database::PgPool;
use crate::utilities::error::AppError;
use log::{debug, error, info};

pub struct RolePermissionRepository<'a> {
  conn: &'a PgPool,
}

impl<'a> RolePermissionRepository<'a> {
  pub fn new(conn: &'a PgPool) -> Self {
    debug!("Creating RolePermissionRepository");
    Self { conn }
  }

  pub fn create(&self, role_id: Uuid, permission_id: Uuid) -> Result<RolePermission, AppError> {
    info!("Creating role_permission in repository: role_id={}, permission_id={}", role_id, permission_id);
    let mut conn = self.conn.get().map_err(|e| {
      error!("Failed to get database connection: {}", e);
      AppError::ConnectionError(format!("Failed to get database connection: {}", e))
    })?;
    let new_role_permission = NewRolePermission {
      role_id,
      permission_id,
    };
    debug!("Inserting role_permission into database: role_id={}, permission_id={}", role_id, permission_id);
    let role_permission: RolePermission = conn.transaction(|conn| {
      diesel::insert_into(role_permissions::table)
        .values(&new_role_permission)
        .get_result(conn)
        .map_err(|e| {
          error!("Failed to create role_permission for role_id={} and permission_id={}: {:?}", role_id, permission_id, e);
          AppError::from(e)
        })
    })?;
    info!("RolePermission created successfully in repository: role_id={}, permission_id={}", role_id, permission_id);
    Ok(role_permission)
  }

  pub fn find_by_ids(&self, role_id: Uuid, permission_id: Uuid) -> Result<RolePermission, AppError> {
    info!("Looking up role_permission by role_id={} and permission_id={}", role_id, permission_id);
    let mut conn = self.conn.get().map_err(|e| {
      error!("Failed to get database connection: {}", e);
      AppError::ConnectionError(format!("Failed to get database connection: {}", e))
    })?;
    debug!("Querying database for role_permission: role_id={}, permission_id={}", role_id, permission_id);
    let role_permission = role_permissions::table
      .filter(role_permissions::role_id.eq(role_id))
      .filter(role_permissions::permission_id.eq(permission_id))
      .first(&mut conn)
      .map_err(|e| {
        error!("Failed to find role_permission with role_id={} and permission_id={}: {:?}", role_id, permission_id, e);
        AppError::from(e)
      })?;
    info!("Found role_permission: role_id={}, permission_id={}", role_id, permission_id);
    Ok(role_permission)
  }

  pub fn find_by_role_id(&self, role_id: Uuid) -> Result<Vec<RolePermission>, AppError> {
    info!("Looking up role_permissions by role_id={}", role_id);
    let mut conn = self.conn.get().map_err(|e| {
      error!("Failed to get database connection: {}", e);
      AppError::ConnectionError(format!("Failed to get database connection: {}", e))
    })?;
    debug!("Querying database for role_permissions with role_id={}", role_id);
    let role_permissions = role_permissions::table
      .filter(role_permissions::role_id.eq(role_id))
      .load::<RolePermission>(&mut conn)
      .map_err(|e| {
        error!("Failed to retrieve role_permissions for role_id={}: {:?}", role_id, e);
        AppError::from(e)
      })?;
    info!("Found {} role_permissions for role_id={}", role_permissions.len(), role_id);
    Ok(role_permissions)
  }

  pub fn delete(&self, role_id: Uuid, permission_id: Uuid) -> Result<(), AppError> {
    info!("Deleting role_permission in repository: role_id={}, permission_id={}", role_id, permission_id);
    let mut conn = self.conn.get().map_err(|e| {
      error!("Failed to get database connection: {}", e);
      AppError::ConnectionError(format!("Failed to get database connection: {}", e))
    })?;
    debug!("Deleting role_permission from database: role_id={}, permission_id={}", role_id, permission_id);
    let affected = conn.transaction(|conn| {
      diesel::delete(
        role_permissions::table
          .filter(role_permissions::role_id.eq(role_id))
          .filter(role_permissions::permission_id.eq(permission_id))
      )
      .execute(conn)
      .map_err(|e| {
        error!("Failed to delete role_permission with role_id={} and permission_id={}: {:?}", role_id, permission_id, e);
        AppError::from(e)
      })
    })?;
    if affected == 0 {
      error!("RolePermission with role_id={} and permission_id={} not found for deletion", role_id, permission_id);
      return Err(AppError::NotFound(format!("RolePermission with role_id={} and permission_id={} not found", role_id, permission_id)));
    }
    info!("RolePermission deleted successfully: role_id={}, permission_id={}", role_id, permission_id);
    Ok(())
  }
}