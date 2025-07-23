use crate::database::PgPool;
use crate::models::user_role::UserRole;
use crate::repositories::user_role::UserRoleRepository;
use crate::utilities::error::AppError;
use log::{debug, info};
use uuid::Uuid;

pub struct UserRoleHandler<'a> {
  repo: UserRoleRepository<'a>,
}

impl<'a> UserRoleHandler<'a> {
  pub fn new(pool: &'a PgPool) -> Self {
    debug!("Creating UserRoleHandler");
    Self {
      repo: UserRoleRepository::new(pool),
    }
  }

  pub fn create(&self, user_id: Uuid, role_id: Uuid) -> Result<UserRole, AppError> {
    info!("Creating user_role: user_id={}, role_id={}", user_id, role_id);
    debug!("Calling UserRoleRepository to create user_role: user_id={}, role_id={}", user_id, role_id);
    let user_role = self.repo.create(user_id, role_id)?;
    info!("UserRole created successfully: user_id={}, role_id={}", user_id, role_id);
    Ok(user_role)
  }

  pub fn find_by_ids(&self, user_id: Uuid, role_id: Uuid) -> Result<UserRole, AppError> {
    info!("Looking up user_role: user_id={}, role_id={}", user_id, role_id);
    debug!("Calling UserRoleRepository to find user_role: user_id={}, role_id={}", user_id, role_id);
    let user_role = self.repo.find_by_ids(user_id, role_id)?;
    info!("Found user_role: user_id={}, role_id={}", user_id, role_id);
    Ok(user_role)
  }

  pub fn find_by_user_id(&self, user_id: Uuid) -> Result<Vec<UserRole>, AppError> {
    info!("Looking up user_roles for user_id={}", user_id);
    debug!("Calling UserRoleRepository to find user_roles for user_id={}", user_id);
    let user_roles = self.repo.find_by_user_id(user_id)?;
    info!("Found {} user_roles for user_id={}", user_roles.len(), user_id);
    Ok(user_roles)
  }

  pub fn delete(&self, user_id: Uuid, role_id: Uuid) -> Result<(), AppError> {
    info!("Deleting user_role: user_id={}, role_id={}", user_id, role_id);
    debug!("Calling UserRoleRepository to delete user_role: user_id={}, role_id={}", user_id, role_id);
    self.repo.delete(user_id, role_id)?;
    info!("UserRole deleted successfully: user_id={}, role_id={}", user_id, role_id);
    Ok(())
  }
}