use diesel::prelude::*;
use uuid::Uuid;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use crate::schema::permissions;

#[derive(Queryable, Identifiable, Debug, Serialize)]
#[diesel(table_name = permissions)]
pub struct Permission {
  pub id: Uuid,
  pub name: String,
  pub description: Option<String>,
  pub created_at: DateTime<Utc>,
  pub updated_at: DateTime<Utc>,
}

#[derive(Insertable, Debug, Deserialize)]
#[diesel(table_name = permissions)]
pub struct NewPermission<'a> {
  pub name: &'a str,
  pub description: Option<&'a str>,
}

#[derive(AsChangeset)]
#[diesel(table_name = crate::schema::permissions)]
pub struct UpdatePermission<'a> {
  pub name: Option<&'a str>,
  pub description: Option<&'a str>,
  pub updated_at: DateTime<Utc>,
}

#[derive(Serialize)]
pub struct PermissionResponse {
  pub id: Uuid,
  pub name: String,
  pub description: Option<String>,
  pub created_at: DateTime<Utc>,
  pub updated_at: DateTime<Utc>,
}

impl From<Permission> for PermissionResponse {
  fn from(permission: Permission) -> Self {
    PermissionResponse {
      id: permission.id,
      name: permission.name,
      description: permission.description,
      created_at: permission.created_at,
      updated_at: permission.updated_at,
    }
  }
}