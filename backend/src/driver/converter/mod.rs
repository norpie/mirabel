use std::fmt::Display;

use html::Html;
use markdown::Markdown;
use pdf::Pdf;

pub mod html;
pub mod pdf;
pub mod markdown;

pub struct Converter<T> {
    data: T,
}

impl<T> Converter<T> {
    pub fn from_html(html: String) -> Converter<Html> {
        Converter { data: Html(html) }
    }

    pub fn from_pdf(pdf: Vec<u8>) -> Converter<Pdf> {
        Converter { data: Pdf(pdf) }
    }

    pub fn from_md(md: String) -> Converter<Markdown> {
        Converter { data: Markdown(md) }
    }
}
