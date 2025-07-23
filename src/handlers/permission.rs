use crate::database::PgPool;
use crate::models::permission::{Permission, NewPermission, UpdatePermission};
use crate::repositories::permission::PermissionRepository;
use crate::utilities::error::AppError;
use log::{debug, info};
use uuid::Uuid;
use chrono::Utc;

pub struct PermissionHandler<'a> {
  repo: PermissionRepository<'a>,
}

impl<'a> PermissionHandler<'a> {
  pub fn new(pool: &'a PgPool) -> Self {
    debug!("Creating PermissionHandler");
    Self {
      repo: PermissionRepository::new(pool),
    }
  }

  pub fn create(&self, name: &str, description: Option<&str>) -> Result<Permission, AppError> {
    info!("Creating permission: {}", name);
    let new_permission = NewPermission {
      name,
      description,
    };
    debug!("Calling PermissionRepository to create permission: {}", name);
    let permission = self.repo.create(new_permission)?;
    info!("Permission created successfully: {}", name);
    Ok(permission)
  }

  pub fn find_by_name(&self, name: &str) -> Result<Permission, AppError> {
    info!("Looking up permission: {}", name);
    debug!("Calling PermissionRepository to find permission: {}", name);
    let permission = self.repo.find_by_name(name)?;
    info!("Found permission: {}", name);
    Ok(permission)
  }

  pub fn find_by_id(&self, id: Uuid) -> Result<Permission, AppError> {
    info!("Looking up permission by ID: {}", id);
    debug!("Calling PermissionRepository to find permission ID: {}", id);
    let permission = self.repo.find_by_id(id)?;
    info!("Found permission by ID: {}", id);
    Ok(permission)
  }

  pub fn update(&self, id: Uuid, name: Option<&str>, description: Option<&str>) -> Result<Permission, AppError> {
    info!("Updating permission: {}", id);
    let update_permission = UpdatePermission {
      name,
      description,
      updated_at: Utc::now(),
    };
    debug!("Calling PermissionRepository to update permission: {}", id);
    let permission = self.repo.update(id, update_permission)?;
    info!("Permission updated successfully: {}", id);
    Ok(permission)
  }

  pub fn delete(&self, id: Uuid) -> Result<(), AppError> {
    info!("Deleting permission: {}", id);
    debug!("Calling PermissionRepository to delete permission: {}", id);
    self.repo.delete(id)?;
    info!("Permission deleted successfully: {}", id);
    Ok(())
  }
}