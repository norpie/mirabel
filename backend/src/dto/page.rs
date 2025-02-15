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

impl Default for PageRequest {
    fn default() -> Self {
        Self { page: 0, size: 20 }
    }
}

impl PageRequest {
    pub fn page(&self) -> i32 {
        self.page
    }

    pub fn size(&self) -> i32 {
        self.size
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PageInfo {
    page: i32,
    size: i32,
    total: i32,
}
