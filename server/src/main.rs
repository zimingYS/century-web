use axum::*;
use server::database::connection::create_pool;
use server::repositories::user_repository::UserRepository;
use server::routes;
use server::services::user_service::UserService;
use server::state::{AppState, SharedAppState};
use std::sync::Arc;

#[tokio::main]
async fn main() {
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

    // 创建网站路由
    let app = Router::new().merge(routes::user::routes()).with_state(state);

    // 监听端口
    let linstener = tokio::net::TcpListener::bind(("127.0.0.1", 3000)).await.unwrap();
    println!("Server is running at 'http://127.0.0.1:3000'");

    // 处理请求响应
    serve(linstener, app).await.unwrap();
}
