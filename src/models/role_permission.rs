use diesel::prelude::*;
use uuid::Uuid;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use crate::schema::role_permissions;

#[derive(Queryable, Identifiable, Debug, Serialize)]
#[diesel(table_name = role_permissions)]
#[diesel(primary_key(role_id, permission_id))]
pub struct RolePermission {
  pub role_id: Uuid,
  pub permission_id: Uuid,
  pub created_at: DateTime<Utc>,
}

#[derive(Insertable, Debug, Deserialize)]
#[diesel(table_name = role_permissions)]
pub struct NewRolePermission {
  pub role_id: Uuid,
  pub permission_id: Uuid,
}