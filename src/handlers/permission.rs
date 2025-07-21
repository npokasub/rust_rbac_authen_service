use crate::models::permission::{Permission, NewPermission};
use crate::repositories::permission::PermissionRepository;
use crate::utilities::error::AppError;
use crate::database::PgPool;

pub struct PermissionHandler<'a> {
  permission_repo: PermissionRepository<'a>,
}

impl<'a> PermissionHandler<'a> {
  pub fn new(pool: &'a PgPool) -> Self {
    PermissionHandler {
      permission_repo: PermissionRepository::new(pool),
    }
  }

  pub fn create(&self, name: &str, description: Option<&str>) -> Result<Permission, AppError> {
    let new_permission = NewPermission { name, description };
    self.permission_repo.create(new_permission)
  }

  pub fn find_by_id(&self, id: uuid::Uuid) -> Result<Permission, AppError> {
    self.permission_repo.find_by_id(id)
  }
}