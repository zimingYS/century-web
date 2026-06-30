use crate::models::user::LoginRequest;
use axum::extract::*;

pub async fn login(Json(request): Json<LoginRequest>) -> String {
    format!("Login: {}", request.username)
}
