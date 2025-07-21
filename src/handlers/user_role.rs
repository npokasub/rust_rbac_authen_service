use crate::models::user_role::{UserRole, NewUserRole};
use crate::repositories::user_role::UserRoleRepository;
use crate::utilities::error::AppError;
use crate::database::PgPool;

pub struct UserRoleHandler<'a> {
  user_role_repo: UserRoleRepository<'a>,
}

impl<'a> UserRoleHandler<'a> {
  pub fn new(pool: &'a PgPool) -> Self {
    UserRoleHandler {
      user_role_repo: UserRoleRepository::new(pool),
    }
  }

  pub fn assign_role(&self, user_id: uuid::Uuid, role_id: uuid::Uuid) -> Result<UserRole, AppError> {
    let new_user_role = NewUserRole { user_id, role_id };
    self.user_role_repo.create(new_user_role)
  }

  pub fn find_by_user_id(&self, user_id: uuid::Uuid) -> Result<Vec<UserRole>, AppError> {
    self.user_role_repo.find_by_user_id(user_id)
  }
}