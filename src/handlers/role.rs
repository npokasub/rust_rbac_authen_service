use crate::models::role::{Role, NewRole};
use crate::repositories::role::RoleRepository;
use crate::utilities::error::AppError;
use crate::database::PgPool;

pub struct RoleHandler<'a> {
  role_repo: RoleRepository<'a>,
}

impl<'a> RoleHandler<'a> {
  pub fn new(pool: &'a PgPool) -> Self {
    RoleHandler {
      role_repo: RoleRepository::new(pool),
    }
  }

  pub fn create(&self, name: &str, description: Option<&str>) -> Result<Role, AppError> {
    let new_role = NewRole { name, description };
    self.role_repo.create(new_role)
  }

  pub fn find_by_id(&self, id: uuid::Uuid) -> Result<Role, AppError> {
    self.role_repo.find_by_id(id)
  }
}