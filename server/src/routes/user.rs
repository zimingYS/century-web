use crate::handlers::user::login;
use crate::state::SharedAppState;
use axum::Router;
use axum::routing::*;

/// 注册所有与用户相关的 HTTP 路由。
///
/// 每个业务模块都应提供一个 `routes()` 函数，
/// 由 `main.rs` 统一进行组装，避免程序入口了解具体业务实现。
pub fn routes() -> Router<SharedAppState> {
    Router::new().route("/login", post(login))
}
