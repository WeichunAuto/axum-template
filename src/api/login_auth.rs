use crate::application::AppState;
use crate::auth::{get_jwt, Principal};
use crate::common::verify_password;
use crate::entity::prelude::*;
use crate::entity::users;
use crate::error::ApiError;
use crate::middleware::get_auth_layer;
use crate::request::BValidJson;
use crate::response::{ApiResponse, ApiResult};
use axum::extract::{ConnectInfo, State};
use axum::routing::{get, post};
use axum::{debug_handler, Extension, Router};
use sea_orm::prelude::*;
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use validator::Validate;

#[derive(Debug, Deserialize, Validate)]
pub struct LoginParams {
    #[validate(custom(
        function = "crate::request::is_email_valid",
        message = "invalid email format, please check."
    ))]
    account: String,
    password: String,
}

#[derive(Debug, Serialize)]
pub struct LoginResponse {
    access_token: String,
}

pub(crate) fn routes() -> Router<AppState> {
    Router::new()
        .route("/get_user_info", get(get_user_info))
        .route_layer(get_auth_layer())
        .route("/login", post(login))
}

#[debug_handler]
#[tracing::instrument(name = "login", skip_all, fields(account = %account, IP = %addr))]
pub async fn login(
    State(AppState { db }): State<AppState>,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    BValidJson(LoginParams { account, password }): BValidJson<LoginParams>,
) -> ApiResult<LoginResponse> {
    tracing::info!("start login, account: {}", account);

    let user = Users::find()
        .filter(users::Column::Email.eq(&account))
        .one(&db)
        .await?
        .ok_or_else(|| {
            tracing::error!("user not found, account: {}", account);
            ApiError::BizError("user or password is not correct!".to_string())
        })?;

    if !verify_password(&password, user.password_hash.as_str())? {
        tracing::error!("password is not correct, account: {}", account);
        return Ok(ApiResponse::error("password is not correct".to_string()));
    }

    let principal = Principal {
        id: user.id.to_string(),
        name: user.fullname,
        email: user.email,
    };
    let access_token = get_jwt().encode(principal)?;

    tracing::info!(
        "login success, IP: {}, access_token: {}",
        addr,
        access_token
    );

    Ok(ApiResponse::success(
        "login success",
        Some(LoginResponse { access_token }),
    ))
}

#[debug_handler]
pub async fn get_user_info(Extension(principal): Extension<Principal>) -> ApiResult<Principal> {
    Ok(ApiResponse::success("", Some(principal)))
}
