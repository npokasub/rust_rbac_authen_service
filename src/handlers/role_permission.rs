use crate::database::PgPool;
use crate::models::role_permission::RolePermission;
use crate::repositories::role_permission::RolePermissionRepository;
use crate::utilities::error::AppError;
use log::{debug, info};
use uuid::Uuid;

pub struct RolePermissionHandler<'a> {
  repo: RolePermissionRepository<'a>,
}

impl<'a> RolePermissionHandler<'a> {
  pub fn new(pool: &'a PgPool) -> Self {
    debug!("Creating RolePermissionHandler");
    Self {
      repo: RolePermissionRepository::new(pool),
    }
  }

  pub fn create(&self, role_id: Uuid, permission_id: Uuid) -> Result<RolePermission, AppError> {
    info!("Creating role_permission: role_id={}, permission_id={}", role_id, permission_id);
    debug!("Calling RolePermissionRepository to create role_permission: role_id={}, permission_id={}", role_id, permission_id);
    let role_permission = self.repo.create(role_id, permission_id)?;
    info!("RolePermission created successfully: role_id={}, permission_id={}", role_id, permission_id);
    Ok(role_permission)
  }

  pub fn find_by_ids(&self, role_id: Uuid, permission_id: Uuid) -> Result<RolePermission, AppError> {
    info!("Looking up role_permission: role_id={}, permission_id={}", role_id, permission_id);
    debug!("Calling RolePermissionRepository to find role_permission: role_id={}, permission_id={}", role_id, permission_id);
    let role_permission = self.repo.find_by_ids(role_id, permission_id)?;
    info!("Found role_permission: role_id={}, permission_id={}", role_id, permission_id);
    Ok(role_permission)
  }

  pub fn find_by_role_id(&self, role_id: Uuid) -> Result<Vec<RolePermission>, AppError> {
    info!("Looking up role_permissions for role_id={}", role_id);
    debug!("Calling RolePermissionRepository to find role_permissions for role_id={}", role_id);
    let role_permissions = self.repo.find_by_role_id(role_id)?;
    info!("Found {} role_permissions for role_id={}", role_permissions.len(), role_id);
    Ok(role_permissions)
  }

  pub fn delete(&self, role_id: Uuid, permission_id: Uuid) -> Result<(), AppError> {
    info!("Deleting role_permission: role_id={}, permission_id={}", role_id, permission_id);
    debug!("Calling RolePermissionRepository to delete role_permission: role_id={}, permission_id={}", role_id, permission_id);
    self.repo.delete(role_id, permission_id)?;
    info!("RolePermission deleted successfully: role_id={}, permission_id={}", role_id, permission_id);
    Ok(())
  }
}