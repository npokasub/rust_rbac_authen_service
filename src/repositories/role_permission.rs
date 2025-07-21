use diesel::prelude::*;
use uuid::Uuid;
use crate::schema::role_permissions;
use crate::models::role_permission::{RolePermission, NewRolePermission};
use crate::database::PgPool;
use crate::utilities::error::AppError;

pub struct RolePermissionRepository<'a> {
  conn: &'a PgPool,
}

impl<'a> RolePermissionRepository<'a> {
  pub fn new(conn: &'a PgPool) -> Self {
    RolePermissionRepository { conn }
  }

  pub fn create(&self, new_role_permission: NewRolePermission) -> Result<RolePermission, AppError> {
    let mut conn = self.conn.get()?;
    diesel::insert_into(role_permissions::table)
      .values(&new_role_permission)
      .get_result(&mut conn)
      .map_err(AppError::from)
  }

  pub fn find_by_role_id(&self, role_id: Uuid) -> Result<Vec<RolePermission>, AppError> {
    let mut conn = self.conn.get()?;
    role_permissions::table
      .filter(role_permissions::role_id.eq(role_id))
      .load(&mut conn)
      .map_err(AppError::from)
  }

  pub fn delete(&self, role_id: Uuid, permission_id: Uuid) -> Result<usize, AppError> {
    let mut conn = self.conn.get()?;
    diesel::delete(
      role_permissions::table
        .filter(role_permissions::role_id.eq(role_id))
        .filter(role_permissions::permission_id.eq(permission_id))
    )
    .execute(&mut conn)
    .map_err(AppError::from)
  }
}