use crate::prelude::*;

use std::fmt::Display;

use super::{markdown::Markdown, Converter};

pub struct Html(pub(super) String);

impl Converter<Html> {
    pub fn to_md(&self) -> Result<Converter<Markdown>> {
        Ok(Converter {
            data: Markdown(html2md::rewrite_html(&self.data.0, false)),
        })
    }
}

impl Display for Converter<Html> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.data.0)
    }
}
