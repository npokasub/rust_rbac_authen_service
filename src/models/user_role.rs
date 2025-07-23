use diesel::prelude::*;
use uuid::Uuid;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use crate::schema::user_roles;

#[derive(Queryable, Identifiable, Debug, Serialize)]
#[diesel(table_name = user_roles)]
#[diesel(primary_key(user_id, role_id))]
pub struct UserRole {
  pub user_id: Uuid,
  pub role_id: Uuid,
  pub created_at: DateTime<Utc>,
}

#[derive(Insertable, Debug, Deserialize)]
#[diesel(table_name = user_roles)]
pub struct NewUserRole {
  pub user_id: Uuid,
  pub role_id: Uuid,
}

#[derive(Serialize)]
pub struct UserRoleResponse {
  pub user_id: Uuid,
  pub role_id: Uuid,
  pub created_at: DateTime<Utc>,
}

impl From<UserRole> for UserRoleResponse {
  fn from(user_role: UserRole) -> Self {
    UserRoleResponse {
      user_id: user_role.user_id,
      role_id: user_role.role_id,
      created_at: user_role.created_at,
    }
  }
}