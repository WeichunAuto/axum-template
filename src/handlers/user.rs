use crate::entity::prelude::*;
use axum::extract::State;
use axum::extract::{Path, Query};
use sea_orm::{prelude::*, Condition};

use crate::application::AppState;
use crate::entity::users;
use crate::entity::users::Model;
use crate::response::ApiResponse;
use serde::Deserialize;

#[derive(Deserialize)]
pub(crate) struct UserQuery {
    pub active: Option<bool>,
}

#[tracing::instrument(name="get_users", skip(state, params), fields(id = %id, name = %name))]
pub(crate) async fn get_users(
    State(state): State<AppState>,
    Path((id, name)): Path<(u32, String)>,
    Query(params): Query<UserQuery>,
) -> ApiResponse<Option<Model>> {
    tracing::info!("start querying users...");
    let db = state.db();

    let user = Users::find()
        .filter(
            Condition::all()
                .add(users::Column::Id.eq(id))
                .add(users::Column::Fullname.eq(name)),
        )
        // .all(&db)
        .one(db)
        .await
        .unwrap();

    tracing::info!("params: {:?}", params.active);

    ApiResponse::success("success", Some(user))
}
