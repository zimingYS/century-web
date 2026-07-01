use crate::errors::AppError;
use crate::models::user::LoginRequest;
use crate::services::user_service::UserService;
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
    // 查询用户
    let user = state
        .user_service
        .find_by_username(&request.username)
        .await?
        .ok_or_else(|| AppError::NotFound(format!("用户 {} 不存在", request.username)))?;

    // 校验密码
    let password_valid = UserService::verify_password(&request.password, &user.password_hash)?;
    if !password_valid {
        return Err(AppError::Internal("用户名或密码错误".to_string()));
    }

    Ok(format!("登录成功:{}", &request.username))
}
