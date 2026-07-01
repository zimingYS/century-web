use axum::Json;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use serde::Serialize;

/// 全局错误处理
#[derive(Debug)]
pub enum AppError {
    /// 资源不存在
    NotFound(String),
    /// 服务器内部错误
    Internal(String),
    /// 数据库错误
    Database(sqlx::Error),
    /// 认证/授权失败
    Unauthorized(String),
}

impl From<sqlx::Error> for AppError {
    fn from(err: sqlx::Error) -> AppError {
        AppError::Database(err)
    }
}

/// 错误响应
#[derive(Serialize)]
struct ErrorResponse {
    error: String,
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        match self {
            AppError::NotFound(message) => {
                (StatusCode::NOT_FOUND, Json(ErrorResponse { error: message })).into_response()
            }
            AppError::Internal(message) => {
                (StatusCode::INTERNAL_SERVER_ERROR, Json(ErrorResponse { error: message }))
                    .into_response()
            }
            AppError::Database(error) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse { error: error.to_string() }),
            )
                .into_response(),
            AppError::Unauthorized(message) => {
                (StatusCode::UNAUTHORIZED, Json(ErrorResponse { error: message })).into_response()
            }
        }
    }
}
