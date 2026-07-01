use crate::errors::AppError;
use crate::models::entity::user_entity::UserEntity;
use crate::repositories::user_repository::UserRepository;
use argon2::password_hash::SaltString;
use argon2::{Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
use rand::rngs::OsRng;

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

    /// 对密码进行加密
    pub fn hash_password(password: &str) -> Result<String, AppError> {
        // 生成随机盐值
        let salt = SaltString::generate(&mut OsRng);

        // 生成哈希密码
        let password_hash = Argon2::default()
            .hash_password(password.as_ref(), &salt)
            .map_err(|e| AppError::Internal(e.to_string()))?;

        Ok(password_hash.to_string())
    }

    /// 验证加密后的密码是否正确
    pub(crate) fn verify_password(password: &str, hashed_password: &str) -> Result<bool, AppError> {
        // 解析哈希字符串
        let parsed_hash =
            PasswordHash::new(hashed_password).map_err(|e| AppError::Internal(e.to_string()))?;

        Ok(Argon2::default().verify_password(password.as_bytes(), &parsed_hash).is_ok())
    }
}
