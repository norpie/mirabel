use std::num::NonZeroI64;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PageResponse<T> {
    #[serde(rename = "pageInfo")]
    page_info: PageInfo,
    data: Vec<T>,
}

impl<T> PageResponse<T> {
    pub fn to<R>(self) -> PageResponse<R>
    where
        T: Into<R>,
    {
        PageResponse {
            page_info: self.page_info,
            data: self.data.into_iter().map(Into::into).collect(),
        }
    }

    pub fn new(page_info: PageInfo, data: Vec<T>) -> Self {
        Self { page_info, data }
    }

    pub fn from_vec(data: Vec<T>, page: NonZeroI64) -> Self {
        let page_info = PageInfo {
            page,
            size: data.len() as i64,
            total: data.len() as i64,
        };
        Self { page_info, data }
    }

    pub fn data(&self) -> &Vec<T> {
        &self.data
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PageRequest {
    page: NonZeroI64,
    size: i64,
}

impl PageRequest {
    pub fn new(page: NonZeroI64, size: i64) -> Self {
        Self { page, size }
    }
}

impl Default for PageRequest {
    fn default() -> Self {
        Self {
            page: NonZeroI64::new(1).unwrap(),
            size: 20,
        }
    }
}

impl PageRequest {
    pub fn page(&self) -> NonZeroI64 {
        self.page
    }

    pub fn size(&self) -> i64 {
        self.size
    }

    pub fn offset(&self) -> i64 {
        (self.page.get() - 1) * self.size
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PageInfo {
    page: NonZeroI64,
    size: i64,
    total: i64,
}

impl PageInfo {
    pub fn new(page: NonZeroI64, size: i64, total: i64) -> Self {
        Self { page, size, total }
    }

    pub fn page(&self) -> NonZeroI64 {
        self.page
    }

    pub fn size(&self) -> i64 {
        self.size
    }

    pub fn total(&self) -> i64 {
        self.total
    }
}
