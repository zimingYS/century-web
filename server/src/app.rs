use crate::database::connection::create_pool;
use crate::repositories::user_repository::UserRepository;
use crate::routes;
use crate::services::user_service::UserService;
use crate::state::{AppState, SharedAppState};
use axum::Router;
use std::sync::Arc;

/// 构建整个应用。
///
/// 负责完成：
///
/// - 初始化数据库
/// - 初始化 Repository
/// - 初始化 Service
/// - 初始化 AppState
/// - 注册所有路由
pub async fn build() -> Router {
    // 获取数据库链接
    dotenvy::dotenv().ok();
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL 未配置");
    // 构造连接池
    let pool = create_pool(&database_url).await.expect("数据库连接失败");
    println!("数据库连接成功");

    // 创建用户服务
    let user_repository = UserRepository::new(pool);
    let user_service = UserService::new(user_repository);

    // 创建全局共享状态
    let state: SharedAppState = Arc::new(AppState { user_service });

    Router::new().merge(routes::user::routes()).with_state(state)
}
