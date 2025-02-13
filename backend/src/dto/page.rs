use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PageResponse<T> {
    #[serde(rename = "pageInfo")]
    page_info: PageInfo,
    data: T,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PageRequest {
    page: i32,
    size: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PageInfo {
    page: i32,
    size: i32,
    total: i32,
}
