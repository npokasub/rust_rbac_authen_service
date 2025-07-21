use diesel::prelude::*;
use uuid::Uuid;
use chrono::Utc;
use crate::schema::permissions;
use crate::models::permission::{Permission, NewPermission};
use crate::database::PgPool;
use crate::utilities::error::AppError;

pub struct PermissionRepository<'a> {
  conn: &'a PgPool,
}

impl<'a> PermissionRepository<'a> {
  pub fn new(conn: &'a PgPool) -> Self {
    PermissionRepository { conn }
  }

  pub fn create(&self, new_permission: NewPermission) -> Result<Permission, AppError> {
    let mut conn = self.conn.get()?;
    diesel::insert_into(permissions::table)
      .values(&new_permission)
      .get_result(&mut conn)
      .map_err(AppError::from)
  }

  pub fn find_by_id(&self, id: Uuid) -> Result<Permission, AppError> {
    let mut conn = self.conn.get()?;
    permissions::table.find(id).first(&mut conn).map_err(AppError::from)
  }

  pub fn find_by_name(&self, name: &str) -> Result<Permission, AppError> {
    let mut conn = self.conn.get()?;
    permissions::table
      .filter(permissions::name.eq(name))
      .first(&mut conn)
      .map_err(AppError::from)
  }

  pub fn update(&self, id: Uuid, new_permission: NewPermission) -> Result<Permission, AppError> {
    let mut conn = self.conn.get()?;
    diesel::update(permissions::table.find(id))
      .set((
        permissions::name.eq(new_permission.name),
        permissions::description.eq(new_permission.description),
        permissions::updated_at.eq(Utc::now()),
      ))
      .get_result(&mut conn)
      .map_err(AppError::from)
  }

  pub fn delete(&self, id: Uuid) -> Result<usize, AppError> {
    let mut conn = self.conn.get()?;
    diesel::delete(permissions::table.find(id))
      .execute(&mut conn)
      .map_err(AppError::from)
  }
}