use diesel::prelude::*;
use uuid::Uuid;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use crate::schema::users;

#[derive(Queryable, Identifiable, Debug, Serialize)]
#[diesel(table_name = users)]
pub struct User {
  pub id: Uuid,
  pub username: String,
  pub email: String,
  pub password_hash: String,
  pub created_at: DateTime<Utc>,
  pub updated_at: DateTime<Utc>,
}

#[derive(Insertable, Debug, Deserialize)]
#[diesel(table_name = users)]
pub struct NewUser<'a> {
  pub username: &'a str,
  pub email: &'a str,
  pub password_hash: &'a str,
}

#[derive(AsChangeset, Debug)]
#[diesel(table_name = users)]
pub struct UpdateUser<'a> {
  pub username: Option<&'a str>,
  pub email: Option<&'a str>,
  pub password_hash: Option<&'a str>,
  pub updated_at: DateTime<Utc>,
}

#[derive(Serialize)]
pub struct UserResponse {
  pub id: Uuid,
  pub username: String,
  pub email: String,
  pub created_at: DateTime<Utc>,
  pub updated_at: DateTime<Utc>,
}

impl From<User> for UserResponse {
  fn from(user: User) -> Self {
    UserResponse {
      id: user.id,
      username: user.username,
      email: user.email,
      created_at: user.created_at,
      updated_at: user.updated_at,     
    }
  }
}