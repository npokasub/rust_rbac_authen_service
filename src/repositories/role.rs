use diesel::prelude::*;
use uuid::Uuid;
use chrono::Utc;
use crate::schema::roles;
use crate::models::role::{Role, NewRole};
use crate::database::PgPool;
use crate::utilities::error::AppError;

pub struct RoleRepository<'a> {
  conn: &'a PgPool,
}

impl<'a> RoleRepository<'a> {
  pub fn new(conn: &'a PgPool) -> Self {
    RoleRepository { conn }
  }

  pub fn create(&self, new_role: NewRole) -> Result<Role, AppError> {
    let mut conn = self.conn.get()?;
    diesel::insert_into(roles::table)
      .values(&new_role)
      .get_result(&mut conn)
      .map_err(AppError::from)
  }

  pub fn find_by_id(&self, id: Uuid) -> Result<Role, AppError> {
    let mut conn = self.conn.get()?;
    roles::table.find(id).first(&mut conn).map_err(AppError::from)
  }

  pub fn find_by_name(&self, name: &str) -> Result<Role, AppError> {
    let mut conn = self.conn.get()?;
    roles::table
        .filter(roles::name.eq(name))
        .first(&mut conn)
        .map_err(AppError::from)
  }

  pub fn update(&self, id: Uuid, new_role: NewRole) -> Result<Role, AppError> {
    let mut conn = self.conn.get()?;
    diesel::update(roles::table.find(id))
      .set((
        roles::name.eq(new_role.name),
        roles::description.eq(new_role.description),
        roles::updated_at.eq(Utc::now()),
      ))
      .get_result(&mut conn)
      .map_err(AppError::from)
  }

  pub fn delete(&self, id: Uuid) -> Result<usize, AppError> {
    let mut conn = self.conn.get()?;
    diesel::delete(roles::table.find(id))
      .execute(&mut conn)
      .map_err(AppError::from)
  }
}