use axum::*;
use server::routes;

#[tokio::main]
async fn main() {
    // 创建网站路由
    let app = Router::new().merge(routes::user::routes());

    // 监听端口
    let linstener = tokio::net::TcpListener::bind(("127.0.0.1", 3000)).await.unwrap();
    println!("Server is running at 'http://127.0.0.1:3000'");

    // 处理请求响应
    serve(linstener, app).await.unwrap();
}
