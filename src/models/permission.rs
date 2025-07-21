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