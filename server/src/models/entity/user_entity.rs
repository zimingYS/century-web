use sqlx::FromRow;
use sqlx::types::chrono::{DateTime, Utc};

/// 数据库中的用户实体
///
/// 与users表一一对应
#[derive(Debug, Clone, FromRow)]
pub struct UserEntity {
    /// 用户主键
    pub id: i64,
    /// 用户名
    pub username: String,
    /// 密码哈希
    pub password_hash: String,
    /// 邮箱
    pub email: String,
    /// 创建时间
    pub created_at: DateTime<Utc>,
    /// 更新时间
    pub updated_at: DateTime<Utc>,
}
