use crate::database::PgPool;
use crate::models::role::{Role, NewRole, UpdateRole, RoleResponse};
use crate::repositories::role::RoleRepository;
use crate::utilities::error::AppError;
use log::{debug, error, info};
use uuid::Uuid;
use chrono::Utc;

pub struct RoleHandler<'a> {
  repo: RoleRepository<'a>,
}

impl<'a> RoleHandler<'a> {
  pub fn new(pool: &'a PgPool) -> Self {
    debug!("Creating RoleHandler");
    Self {
      repo: RoleRepository::new(pool),
    }
  }

  pub fn create(&self, name: &str, description: Option<&str>) -> Result<Role, AppError> {
    info!("Creating role: {}", name);
    let new_role = NewRole {
      name,
      description,
    };
    debug!("Calling RoleRepository to create role: {}", name);
    let role = self.repo.create(new_role)?;
    info!("Role created successfully: {}", name);
    Ok(role)
  }

  pub fn find_by_name(&self, name: &str) -> Result<Role, AppError> {
    info!("Looking up role: {}", name);
    debug!("Calling RoleRepository to find role: {}", name);
    let role = self.repo.find_by_name(name)?;
    info!("Found role: {}", name);
    Ok(role)
  }

  pub fn find_by_id(&self, id: Uuid) -> Result<Role, AppError> {
    info!("Looking up role by ID: {}", id);
    debug!("Calling RoleRepository to find role ID: {}", id);
    let role = self.repo.find_by_id(id)?;
    info!("Found role by ID: {}", id);
    Ok(role)
  }

  pub fn update(&self, id: Uuid, name: Option<&str>, description: Option<&str>) -> Result<Role, AppError> {
    info!("Updating role: {}", id);
    let update_role = UpdateRole {
      name,
      description,
      updated_at: Utc::now(),
    };
    debug!("Calling RoleRepository to update role: {}", id);
    let role = self.repo.update(id, update_role)?;
    info!("Role updated successfully: {}", id);
    Ok(role)
  }

  pub fn delete(&self, id: Uuid) -> Result<(), AppError> {
    info!("Deleting role: {}", id);
    debug!("Calling RoleRepository to delete role: {}", id);
    self.repo.delete(id)?;
    info!("Role deleted successfully: {}", id);
    Ok(())
  }
}