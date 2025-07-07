use crate::prelude::*;

use std::io::{BufWriter, Write};

use pdf_extract::{Document, HTMLOutput};

use super::{Converter, html::Html};

pub struct Pdf(pub(super) Vec<u8>);

impl Converter<Pdf> {
    pub fn to_html(&self) -> Result<Converter<Html>> {
        let doc = Document::load_mem(&self.data.0)?;
        let mut buffer = Vec::new();
        {
            let mut writer = BufWriter::new(&mut buffer);
            let mut output = HTMLOutput::new(&mut writer);
            pdf_extract::output_doc(&doc, &mut output)?;
            writer.flush()?;
        }
        Ok(Converter {
            data: Html(String::from_utf8(buffer)?),
        })
    }
}
