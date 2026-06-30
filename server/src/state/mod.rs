use std::sync::Arc;

/// 全局应用状态。
///
/// 所有需要跨请求共享的资源，都应该放入 AppState。
/// 例如：
///
/// - 数据库连接池
/// - Redis 客户端
/// - JWT 配置
/// - 应用配置
///
/// AppState 在程序启动时创建一次，
/// 随后共享给所有 Handler。
pub struct AppState {
    /// 当前应用名称
    pub app_name: String,
}

/// AppState 的共享类型。
///
/// 使用 Arc 实现线程安全的共享所有权，
/// 避免复制大型资源。
pub type SharedAppState = Arc<AppState>;
