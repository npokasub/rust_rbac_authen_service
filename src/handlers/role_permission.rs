use crate::models::role_permission::{RolePermission, NewRolePermission};
use crate::repositories::role_permission::RolePermissionRepository;
use crate::utilities::error::AppError;
use crate::database::PgPool;

pub struct RolePermissionHandler<'a> {
  role_permission_repo: RolePermissionRepository<'a>,
}

impl<'a> RolePermissionHandler<'a> {
  pub fn new(pool: &'a PgPool) -> Self {
    RolePermissionHandler {
      role_permission_repo: RolePermissionRepository::new(pool),
    }
  }

  pub fn assign_permission(&self, role_id: uuid::Uuid, permission_id: uuid::Uuid) -> Result<RolePermission, AppError> {
    let new_role_permission = NewRolePermission { role_id, permission_id };
    self.role_permission_repo.create(new_role_permission)
  }

  pub fn find_by_role_id(&self, role_id: uuid::Uuid) -> Result<Vec<RolePermission>, AppError> {
    self.role_permission_repo.find_by_role_id(role_id)
  }
}