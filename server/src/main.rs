use axum::*;
use server::routes;
use server::state::{AppState, SharedAppState};
use std::sync::Arc;

#[tokio::main]
async fn main() {
    // 创建全局共享状态。
    let state: SharedAppState = Arc::new(AppState { app_name: "TaskFlow".to_string() });

    // 创建网站路由
    let app = Router::new().merge(routes::user::routes()).with_state(state);

    // 监听端口
    let linstener = tokio::net::TcpListener::bind(("127.0.0.1", 3000)).await.unwrap();
    println!("Server is running at 'http://127.0.0.1:3000'");

    // 处理请求响应
    serve(linstener, app).await.unwrap();
}
