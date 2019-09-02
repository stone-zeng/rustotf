use crate::font::{Font, TableRecord};
use crate::util::Buffer;

/// ## `post` &mdash; PostScript Table
///
/// <https://docs.microsoft.com/en-us/typography/opentype/spec/post>
///
/// This table contains additional information needed to use TrueType or
/// OpenType fonts on PostScript printers. This includes data for the FontInfo
/// dictionary entry and the PostScript names of all the glyphs.

#[allow(non_camel_case_types)]
#[derive(Debug)]
pub struct Table_post {}

impl Font {
    pub fn parse_post(&mut self, buffer: &mut Buffer, record: &TableRecord) {
        buffer.offset = record.offset;
    }
}
