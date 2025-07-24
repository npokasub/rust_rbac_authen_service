use diesel::prelude::*;
use uuid::Uuid;
use crate::schema::roles;
use crate::models::role::{Role, NewRole, UpdateRole};
use crate::database::PgPool;
use crate::utilities::error::AppError;
use log::{debug, error, info};

pub struct RoleRepository<'a> {
  conn: &'a PgPool,
}

impl<'a> RoleRepository<'a> {
  pub fn new(conn: &'a PgPool) -> Self {
    debug!("Creating RoleRepository");
    Self { conn }
  }

  pub fn create(&self, new_role: NewRole) -> Result<Role, AppError> {
    info!("Creating role in repository: {}", new_role.name);
    let mut conn = self.conn.get().map_err(|e| {
      error!("Failed to get database connection: {}", e);
      AppError::ConnectionError(format!("Failed to get database connection: {}", e))
    })?;
    debug!("Inserting role into database: {}", new_role.name);
    let role: Role = conn.transaction(|conn| {
      diesel::insert_into(roles::table)
        .values(&new_role)
        .get_result(conn)
        .map_err(|e| {
          error!("Failed to create role {}: {:?}", new_role.name, e);
          AppError::from(e)
        })
    })?;
    info!("Role created successfully in repository: {}", role.name);
    Ok(role)
  }

  pub fn find_by_id(&self, id: Uuid) -> Result<Role, AppError> {
    info!("Looking up role by ID in repository: {}", id);
    let mut conn = self.conn.get().map_err(|e| {
      error!("Failed to get database connection: {}", e);
      AppError::ConnectionError(format!("Failed to get database connection: {}", e))
    })?;
    debug!("Querying database for role ID: {}", id);
    let role = roles::table
      .find(id)
      .first(&mut conn)
      .map_err(|e| {
        error!("Failed to find role with ID {}: {:?}", id, e);
        AppError::from(e)
      })?;
    info!("Found role by ID in repository: {}", id);
    Ok(role)
  }

  pub fn find_by_name(&self, name: &str) -> Result<Role, AppError> {
    info!("Looking up role by name in repository: {}", name);
    let mut conn = self.conn.get().map_err(|e| {
      error!("Failed to get database connection: {}", e);
      AppError::ConnectionError(format!("Failed to get database connection: {}", e))
    })?;
    debug!("Querying database for role: {}", name);
    let role = roles::table
      .filter(roles::name.eq(name))
      .first(&mut conn)
      .map_err(|e| {
        error!("Failed to find role with name {}: {:?}", name, e);
        AppError::from(e)
      })?;
    info!("Found role by name in repository: {}", name);
    Ok(role)
  }

  pub fn update(&self, id: Uuid, update_role: UpdateRole) -> Result<Role, AppError> {
    info!("Updating role in repository: {}", id);
    let mut conn = self.conn.get().map_err(|e| {
      error!("Failed to get database connection: {}", e);
      AppError::ConnectionError(format!("Failed to get database connection: {}", e))
    })?;
    debug!("Updating role in database: {}", id);
    let role = conn.transaction(|conn| {
      diesel::update(roles::table.find(id))
        .set(&update_role)
        .get_result(conn)
        .map_err(|e| {
          error!("Failed to update role with ID {}: {:?}", id, e);
          AppError::from(e)
        })
    })?;
    info!("Role updated successfully in repository: {}", id);
    Ok(role)
  }

  pub fn delete(&self, id: Uuid) -> Result<(), AppError> {
    info!("Deleting role in repository: {}", id);
    let mut conn = self.conn.get().map_err(|e| {
      error!("Failed to get database connection: {}", e);
      AppError::ConnectionError(format!("Failed to get database connection: {}", e))
    })?;
    debug!("Deleting role from database: {}", id);
    let affected = conn.transaction(|conn| {
      diesel::delete(roles::table.find(id))
        .execute(conn)
        .map_err(|e| {
          error!("Failed to delete role with ID {}: {:?}", id, e);
          AppError::from(e)
        })
    })?;
    if affected == 0 {
      error!("Role with ID {} not found for deletion", id);
      return Err(AppError::NotFound(format!("Role with ID {} not found", id)));
    }
    info!("Role deleted successfully in repository: {}", id);
    Ok(())
  }
}