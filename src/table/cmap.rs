use crate::font::{Font, TableRecord};
use crate::util::Buffer;

/// ## `cmap` &mdash; Character to Glyph Index Mapping Table
///
/// <https://docs.microsoft.com/en-us/typography/opentype/spec/cmap>
///
/// This table defines the mapping of character codes to the glyph index values
/// used in the font. It may contain more than one subtable, in order to support
/// more than one character encoding scheme.

#[allow(non_camel_case_types)]
#[derive(Debug)]
pub struct Table_cmap {}

impl Font {
    pub fn parse_cmap(&mut self, buffer: &mut Buffer, record: &TableRecord) {
        buffer.offset = record.offset;
    }
}
