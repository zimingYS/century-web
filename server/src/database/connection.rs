use sqlx::PgPool;
use sqlx::postgres::PgPoolOptions;

/// 创建 PostgreSQL 数据库连接池。
///
/// # 参数
/// * `database_url` - PostgreSQL 连接字符串。
///
/// # 返回值
/// 成功时返回数据库连接池；失败时返回 SQLx 错误。
pub async fn create_pool(database_url: &str) -> Result<PgPool, sqlx::Error> {
    PgPoolOptions::new()
        // 最大连接数
        .max_connections(5)
        // 建立数据库连接
        .connect(&database_url)
        .await
}
