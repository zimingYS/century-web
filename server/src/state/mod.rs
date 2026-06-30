use sqlx::PgPool;
use std::sync::Arc;

/// 全局共享状态。
///
/// 所有需要跨请求共享的资源都放在这里，例如：
///
/// - 数据库连接池
/// - Redis 客户端
/// - 配置对象
/// - 日志组件
///
/// AppState 在程序启动时创建一次，
/// 然后通过 Arc 在整个应用中共享。
pub struct AppState {
    /// 数据库连接池
    pub db: PgPool,
}

/// AppState 的共享类型。
///
/// 使用 Arc 实现线程安全的共享所有权，
/// 避免复制大型资源。
pub type SharedAppState = Arc<AppState>;
