use crate::entity::prelude::*;
use axum::extract::State;
use axum::extract::{Path, Query};
use axum::Json;
use sea_orm::{prelude::*, Condition, Set};
use std::fmt::{Display, Formatter};

use crate::application::AppState;
use crate::entity::users;
use crate::entity::users::{ActiveModel, Model};
use crate::response::ApiResponse;
use serde::Deserialize;

#[derive(Deserialize)]
pub(crate) struct CreateUserRequest {
    pub fullname: String,
    pub email: String,
    pub password: String,
    pub ws_id: u64,
}

#[derive(Debug, Deserialize)]
pub(crate) struct UserQuery {
    pub id: Option<u64>,
    pub name: Option<String>,
}

/// delete user by id
#[tracing::instrument(name = "delete_user_by_id", skip(db))]
pub(crate) async fn delete_by_id(
    State(AppState { db }): State<AppState>,
    Path(id): Path<u64>,
) -> ApiResponse<()> {
    let rt = users::Entity::delete_by_id(id as i64).exec(&db).await;

    match rt {
        Ok(deleted_user) => {
            if deleted_user.rows_affected > 0 {
                tracing::info!("User was deleted successfully with id = : {:?}!", id);
                ApiResponse::success("User was deleted successfully!", None)
            } else {
                tracing::error!("When delete the user, with id = : {:?} not found", id);
                ApiResponse::error(format!("User with id = : {:?} not found", id))
            }
        }
        Err(e) => {
            tracing::error!("error deleting user: {:?}", e);
            ApiResponse::error(format!("error deleting user: {:?}", e))
        }
    }
}

/// update user ws_id by id
#[tracing::instrument(name = "update_ws_by_id", skip(state))]
pub(crate) async fn update_ws_by_id(
    State(state): State<AppState>,
    Path((id, ws_id)): Path<(u64, u64)>,
) -> ApiResponse<Model> {
    let db = state.db();

    let rt = users::Entity::update(users::ActiveModel {
        id: Set(id as i64),
        ws_id: Set(ws_id as i64),
        ..Default::default()
    })
    .exec(db)
    .await;

    match rt {
        Ok(user) => {
            tracing::info!(
                "user updated successfully with id = : {:?}, name = : {:?}",
                user.id,
                user.fullname
            );
            ApiResponse::success("User updated successfully!", Some(user))
        }
        Err(DbErr::RecordNotUpdated) => {
            tracing::error!("User id: {} not found", id);
            ApiResponse::error(format!("User id: {} not found", id))
        }
        Err(e) => {
            tracing::error!("error updating user: {:?}", e);
            ApiResponse::error(format!("error updating user: {:?}", e))
        }
    }
}

/// create user
#[tracing::instrument(name="create_user", skip(state), fields(user_data = %user_data))]
pub(crate) async fn create(
    State(state): State<AppState>,
    Json(user_data): Json<CreateUserRequest>,
) -> ApiResponse<Model> {
    let db = state.db();

    let existing_user = Users::find()
        .filter(users::Column::Email.eq(&user_data.email))
        .one(db)
        .await
        .unwrap();

    if existing_user.is_some() {
        tracing::warn!("user with email {} already exists", &user_data.email);
        return ApiResponse::error(format!(
            "user with email ({}) already exists",
            &user_data.email
        ));
    }

    let new_user = ActiveModel {
        fullname: Set(user_data.fullname),
        email: Set(user_data.email),
        password_hash: Set(user_data.password),
        ws_id: Set(user_data.ws_id as i64),
        ..Default::default()
    };

    let rt = new_user.insert(db).await;

    match rt {
        Ok(user) => {
            tracing::info!(
                "user created successfully with id = : {:?} and name = : {:?}",
                user.id,
                user.fullname
            );
            ApiResponse::success("User created successfully!", Some(user))
        }
        Err(e) => {
            tracing::error!("error creating user: {:?}", e);
            ApiResponse::error(format!("error creating user: {:?}", e))
        }
    }
}

/// query user by id and name
#[tracing::instrument(name="get_user", skip(state), fields(UserQuery = %params))]
pub(crate) async fn query(
    State(state): State<AppState>,
    Query(params): Query<UserQuery>,
) -> ApiResponse<Vec<Model>> {
    let db = state.db();

    let mut conditions = Condition::all();

    if let Some(id) = params.id {
        conditions = conditions.add(users::Column::Id.eq(id));
    }
    if let Some(name) = params.name {
        conditions = conditions.add(users::Column::Fullname.eq(name));
    }

    let users = Users::find().filter(conditions).all(db).await.unwrap();
    tracing::info!("query users results: {:?}", users);
    ApiResponse::success("success", Some(users))
}

impl Display for CreateUserRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "CreateUserRequest {{ fullname: {}, email: {}, password: ****, ws_id: {} }}",
            self.fullname, self.email, self.ws_id
        )
    }
}

impl Display for UserQuery {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "UserQuery {{ id: {:?}, name: {:?}}}", self.id, self.name)
    }
}
