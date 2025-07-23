use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Queryable, Identifiable, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::roles)]
pub struct Role {
  pub id: Uuid,
  pub name: String,
  pub description: Option<String>,
  pub created_at: DateTime<Utc>,
  pub updated_at: DateTime<Utc>,
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::roles)]
pub struct NewRole<'a> {
  pub name: &'a str,
  pub description: Option<&'a str>,
}

#[derive(AsChangeset)]
#[diesel(table_name = crate::schema::roles)]
pub struct UpdateRole<'a> {
  pub name: Option<&'a str>,
  pub description: Option<&'a str>,
  pub updated_at: DateTime<Utc>,
}

#[derive(Serialize)]
pub struct RoleResponse {
  pub id: Uuid,
  pub name: String,
  pub description: Option<String>,
  pub created_at: DateTime<Utc>,
  pub updated_at: DateTime<Utc>,
}

impl From<Role> for RoleResponse {
  fn from(role: Role) -> Self {
    RoleResponse {
      id: role.id,
      name: role.name,
      description: role.description,
      created_at: role.created_at,
      updated_at: role.updated_at,
    }
  }
}