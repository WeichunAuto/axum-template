use crate::error::ApiError;
use axum::extract::{FromRequest, FromRequestParts};
use axum_valid::HasValidate;

#[derive(Debug, Clone, Copy, Default, FromRequestParts)]
#[from_request(via(axum::extract::Query), rejection(ApiError))]
pub struct BQuery<T>(pub T);

#[derive(Debug, Clone, Copy, Default, FromRequestParts)]
#[from_request(via(axum::extract::Path), rejection(ApiError))]
pub struct BPath<T>(pub T);

#[derive(Debug, Clone, Copy, Default, FromRequest)]
#[from_request(via(axum::extract::Json), rejection(ApiError))]
pub struct BJson<T>(pub T);

impl<T> HasValidate for BQuery<T> {
    type Validate = T;

    fn get_validate(&self) -> &Self::Validate {
        &self.0
    }
}

impl<T> HasValidate for BPath<T> {
    type Validate = T;
    fn get_validate(&self) -> &Self::Validate {
        &self.0
    }
}

impl<T> HasValidate for BJson<T> {
    type Validate = T;
    fn get_validate(&self) -> &Self::Validate {
        &self.0
    }
}
