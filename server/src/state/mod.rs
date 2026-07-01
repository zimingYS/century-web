use crate::services::user_service::UserService;
use std::sync::Arc;

/// 全局共享状态。
///
/// 在程序启动时统一初始化，
/// 然后通过 Arc 在整个应用生命周期中共享。
///
/// AppState 不直接保存数据库连接，
/// 而是保存各业务 Service。
pub struct AppState {
    /// 用户业务服务
    pub user_service: UserService,
}

/// AppState 的共享类型。
///
/// 使用 Arc 实现线程安全的共享所有权，
/// 避免复制大型资源。
pub type SharedAppState = Arc<AppState>;
