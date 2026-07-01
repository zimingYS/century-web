use crate::errors::AppError;
use crate::models::entity::user_entity::UserEntity;
use crate::repositories::user_repository::UserRepository;

/// 用户相关业务服务
pub struct UserService {
    repository: UserRepository,
}

impl UserService {
    /// 创建用户服务
    pub fn new(repository: UserRepository) -> Self {
        Self { repository }
    }

    /// 根据用户名查询用户
    pub async fn find_by_username(&self, username: &str) -> Result<Option<UserEntity>, AppError> {
        self.repository.find_by_username(username).await
    }
}
