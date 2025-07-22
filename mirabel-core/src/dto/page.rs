use std::num::NonZeroI64;

use chrono::DateTime;
use chrono::Utc;
use serde::Deserialize;
use serde::Serialize;

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
            size: 10,
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CursorPageRequest {
    pub before: Option<DateTime<Utc>>,
    pub after: Option<DateTime<Utc>>,
    pub limit: i64,
}

impl Default for CursorPageRequest {
    fn default() -> Self {
        Self {
            before: None,
            after: None,
            limit: 50, // Increased from 20 to ensure scrollbar on initial load
        }
    }
}

impl CursorPageRequest {
    pub fn new(before: Option<DateTime<Utc>>, after: Option<DateTime<Utc>>, limit: i64) -> Self {
        Self {
            before,
            after,
            limit,
        }
    }

    pub fn before(before: DateTime<Utc>, limit: i64) -> Self {
        Self {
            before: Some(before),
            after: None,
            limit,
        }
    }

    pub fn after(after: DateTime<Utc>, limit: i64) -> Self {
        Self {
            before: None,
            after: Some(after),
            limit,
        }
    }

    pub fn limit(limit: i64) -> Self {
        Self {
            before: None,
            after: None,
            limit,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CursorPageResponse<T> {
    pub data: Vec<T>,
    #[serde(rename = "hasMore")]
    pub has_more: bool,
    #[serde(rename = "nextCursor")]
    pub next_cursor: Option<DateTime<Utc>>,
    #[serde(rename = "prevCursor")]
    pub prev_cursor: Option<DateTime<Utc>>,
}

impl<T> CursorPageResponse<T> {
    pub fn new(
        data: Vec<T>,
        has_more: bool,
        next_cursor: Option<DateTime<Utc>>,
        prev_cursor: Option<DateTime<Utc>>,
    ) -> Self {
        Self {
            data,
            has_more,
            next_cursor,
            prev_cursor,
        }
    }

    pub fn to<R>(self) -> CursorPageResponse<R>
    where
        T: Into<R>,
    {
        CursorPageResponse {
            data: self.data.into_iter().map(Into::into).collect(),
            has_more: self.has_more,
            next_cursor: self.next_cursor,
            prev_cursor: self.prev_cursor,
        }
    }
}
