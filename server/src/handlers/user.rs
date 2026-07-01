use crate::errors::AppError;
use crate::models::user::LoginRequest;
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
    State(state): State<SharedAppState>,
    Json(request): Json<LoginRequest>,
) -> Result<String, AppError> {
    let user = state.user_service.find_by_username(&request.username).await?;

    match user {
        Some(user) => Ok(format!("找到用户：{}", user.username)),
        None => Err(AppError::NotFound(format!("用户 {} 不存在", request.username))),
    }
}
