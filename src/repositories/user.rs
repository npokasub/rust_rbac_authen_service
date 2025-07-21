use diesel::prelude::*;
use uuid::Uuid;
use chrono::Utc;
use crate::schema::users;
use crate::models::user::{User, NewUser};
use crate::database::PgPool;
use crate::utilities::error::AppError;

pub struct UserRepository<'a> {
  conn: &'a PgPool,
}

impl<'a> UserRepository<'a> {
  pub fn new(conn: &'a PgPool) -> Self {
    UserRepository { conn }
  }

  pub fn create(&self, new_user: NewUser) -> Result<User, AppError> {
    let mut conn = self.conn.get()?;
    diesel::insert_into(users::table)
      .values(&new_user)
      .get_result(&mut conn)
      .map_err(AppError::from)
  }

  pub fn find_by_id(&self, id: Uuid) -> Result<User, AppError> {
    let mut conn = self.conn.get()?;
    users::table.find(id).first(&mut conn).map_err(AppError::from)
  }

  pub fn find_by_username(&self, username: &str) -> Result<User, AppError> {
    let mut conn = self.conn.get()?;
    users::table
      .filter(users::username.eq(username))
      .first(&mut conn)
      .map_err(AppError::from)
  }

  pub fn update(&self, id: Uuid, new_user: NewUser) -> Result<User, AppError> {
    let mut conn = self.conn.get()?;
    diesel::update(users::table.find(id))
      .set((
        users::username.eq(new_user.username),
        users::email.eq(new_user.email),
        users::password_hash.eq(new_user.password_hash),
        users::updated_at.eq(Utc::now()),
      ))
      .get_result(&mut conn)
      .map_err(AppError::from)
  }

  pub fn delete(&self, id: Uuid) -> Result<usize, AppError> {
    let mut conn = self.conn.get()?;
    diesel::delete(users::table.find(id))
      .execute(&mut conn)
      .map_err(AppError::from)
  }
}