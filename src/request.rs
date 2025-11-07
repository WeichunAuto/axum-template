use crate::error::ApiError;
use axum::extract::{FromRequest, FromRequestParts, Request};
use axum_valid::HasValidate;
use http::request::Parts;
use regex::Regex;
use std::borrow::Cow;
use std::collections::HashMap;
use std::sync::LazyLock;
use validator::ValidationError;

#[derive(Debug, Clone, Copy, Default, FromRequestParts)]
#[from_request(via(axum::extract::Query), rejection(ApiError))]
pub struct BQuery<T>(pub T);

#[derive(Debug, Clone, Copy, Default, FromRequestParts)]
#[from_request(via(axum::extract::Path), rejection(ApiError))]
pub struct BPath<T>(pub T);

#[derive(Debug, Clone, Copy, Default, FromRequest)]
#[from_request(via(axum::extract::Json), rejection(ApiError))]
pub struct BJson<T>(pub T);

#[derive(Debug, Clone, Copy, Default, FromRequestParts, FromRequest)]
#[from_request(via(axum_valid::Valid), rejection(ApiError))]
pub struct BValid<T>(pub T);

#[derive(Debug, Clone, Copy, Default)]
pub struct BValidQuery<T>(pub T);

#[derive(Debug, Clone, Copy, Default)]
pub struct BValidPath<T>(pub T);

#[derive(Debug, Clone, Copy, Default)]
pub struct BValidJson<T>(pub T);

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

macro_rules! impl_from_request {
    ($name:ident, $wrapper: ident, FromRequestParts) => {
        impl<S, T> FromRequestParts<S> for $name<T>
        where
            S: Send + Sync,
            BValid<$wrapper<T>>: FromRequestParts<S, Rejection = ApiError>,
        {
            type Rejection = ApiError;

            async fn from_request_parts(
                parts: &mut Parts,
                state: &S,
            ) -> Result<Self, Self::Rejection> {
                let result = BValid::from_request_parts(parts, state).await?;

                Ok($name(result.0 .0))
            }
        }
    };

    ($name:ident, $wrapper: ident, FromRequest) => {
        impl<S, T> FromRequest<S> for $name<T>
        where
            S: Send + Sync,
            BValid<$wrapper<T>>: FromRequest<S, Rejection = ApiError>,
        {
            type Rejection = ApiError;
            async fn from_request(request: Request, state: &S) -> Result<Self, Self::Rejection> {
                Ok($name(BValid::from_request(request, state).await?.0 .0))
            }
        }
    };
}

impl_from_request!(BValidQuery, BQuery, FromRequestParts);
impl_from_request!(BValidPath, BPath, FromRequest);
impl_from_request!(BValidJson, BJson, FromRequest);

static EMAIL_REGEX: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"^[a-zA-Z0-9_.+-]+@[a-zA-Z0-9-]+\.[a-zA-Z0-9-.]+$").expect("Invalid email regex")
});

pub fn is_email_valid(value: &str) -> Result<(), ValidationError> {
    if EMAIL_REGEX.is_match(value) {
        Ok(())
    } else {
        Err(ValidationError {
            code: Cow::from(""),
            message: Some(Cow::from("invalid email format.")),
            params: HashMap::new(),
        })
    }
}
