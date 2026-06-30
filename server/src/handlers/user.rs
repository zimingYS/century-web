use crate::models::user::{LoginRequest, LoginResponse};
use crate::state::SharedAppState;
use axum::extract::*;

/// 处理用户登录请求。
///
/// # 请求
/// POST /login
///
/// # Body
/// {
///     "username": "...",
///     "password": "..."
/// }
///
/// 当前阶段仅返回用户名，后续将接入数据库、
/// 密码校验以及 JWT Token 生成逻辑。
pub async fn login(
    State(_state): State<SharedAppState>,
    Json(request): Json<LoginRequest>,
) -> Json<LoginResponse> {
    Json(LoginResponse { id: 1, username: request.username })
}
