use axum::*;

#[tokio::main]
async fn main() {
    // 创建网站路由
    let app = Router::new()
        .route("/", routing::get(home))
        .route("/about", routing::get(about))
        .route("/login", routing::post(login));

    // 监听端口
    let linstener = tokio::net::TcpListener::bind(("127.0.0.1", 3000))
        .await
        .unwrap();
    println!("Server is running at 'http://127.0.0.1:3000'");

    // 处理请求响应
    serve(linstener, app).await.unwrap();
}

async fn home() -> &'static str {
    "Home Page"
}

async fn about() -> &'static str {
    "About Page"
}

async fn login() -> &'static str {
    "Login Success"
}
