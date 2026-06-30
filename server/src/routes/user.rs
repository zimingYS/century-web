use crate::handlers::user::login;
use axum::Router;
use axum::routing::*;

pub fn routes() -> Router {
    Router::new().route("/login", post(login))
}
