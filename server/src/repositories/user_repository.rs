use crate::models::entity::user_entity::UserEntity;
use sqlx::PgPool;

/// 用户数据访问层。
///
/// Repository 只负责数据库读写，
/// 不包含任何业务逻辑。
pub struct UserRepository {
    pool: PgPool,
}
impl UserRepository {
    /// 创建 Repository。
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn create() {}
    pub async fn find_by_id() {}

    /// 根据用户名查询用户。
    ///
    /// 如果用户不存在，则返回 None。
    pub async fn find_by_username(
        &self,
        username: &str,
    ) -> Result<Option<UserEntity>, sqlx::Error> {
        let user = sqlx::query_as::<_, UserEntity>(
            r#" SELECT id, username, password_hash, email, created_at, updated_at FROM users WHERE username = $1 "#
        ).bind(username).fetch_optional(&self.pool) .await?;

        Ok(user)
    }
    pub async fn update() {}
    pub async fn delete() {}
}
