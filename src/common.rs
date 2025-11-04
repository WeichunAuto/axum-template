use serde::{Deserialize, Deserializer, Serialize};
use std::fmt::Display;
use std::str::FromStr;

const DEFAULT_PAGE_NUMBER: u64 = 1;
const DEFAULT_PAGE_SIZE: u64 = 2;
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct Pagination {
    #[serde(
        default = "default_page_number",
        deserialize_with = "deserialize_number"
    )]
    pub page: u64,
    #[serde(default = "default_page_size", deserialize_with = "deserialize_number")]
    pub size: u64,
}

/// page response
#[derive(Debug, Serialize)]
pub struct Page<T> {
    pub data: Vec<T>,
    pub total: u64,
    pub page: u64,
    pub size: u64,
}

#[derive(Deserialize)]
#[serde(untagged)]
enum StringOrNumber<T> {
    String(String),
    Number(T),
}

impl<T> Page<T> {
    pub fn new(data: Vec<T>, total: u64, page: u64, size: u64) -> Self {
        Self {
            data,
            total,
            page,
            size,
        }
    }

    pub fn from_pagination(pagination: &Pagination, total: u64, data: Vec<T>) -> Self {
        Self::new(data, total, pagination.page, pagination.size)
    }
}
pub fn deserialize_number<'de, T, D>(deserializer: D) -> Result<T, D::Error>
where
    T: FromStr + Deserialize<'de>,
    T::Err: Display,
    D: Deserializer<'de>,
{
    let string_or_number = StringOrNumber::<T>::deserialize(deserializer)?;
    match string_or_number {
        StringOrNumber::String(s) => s.parse().map_err(serde::de::Error::custom),
        StringOrNumber::Number(n) => Ok(n),
    }
}

fn default_page_number() -> u64 {
    DEFAULT_PAGE_NUMBER
}

fn default_page_size() -> u64 {
    DEFAULT_PAGE_SIZE
}
