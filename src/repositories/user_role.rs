use diesel::prelude::*;
use uuid::Uuid;
use crate::schema::user_roles;
use crate::models::user_role::{UserRole, NewUserRole};
use crate::database::PgPool;
use crate::utilities::error::AppError;

pub struct UserRoleRepository<'a> {
  conn: &'a PgPool,
}

impl<'a> UserRoleRepository<'a> {
  pub fn new(conn: &'a PgPool) -> Self {
    UserRoleRepository { conn }
  }

  pub fn create(&self, new_user_role: NewUserRole) -> Result<UserRole, AppError> {
    let mut conn = self.conn.get()?;
    diesel::insert_into(user_roles::table)
      .values(&new_user_role)
      .get_result(&mut conn)
      .map_err(AppError::from)
  }

  pub fn find_by_user_id(&self, user_id: Uuid) -> Result<Vec<UserRole>, AppError> {
    let mut conn = self.conn.get()?;
    user_roles::table
      .filter(user_roles::user_id.eq(user_id))
      .load(&mut conn)
      .map_err(AppError::from)
  }

  pub fn delete(&self, user_id: Uuid, role_id: Uuid) -> Result<usize, AppError> {
    let mut conn = self.conn.get()?;
    diesel::delete(
      user_roles::table
        .filter(user_roles::user_id.eq(user_id))
        .filter(user_roles::role_id.eq(role_id))
    )
    .execute(&mut conn)
    .map_err(AppError::from)
  }
}