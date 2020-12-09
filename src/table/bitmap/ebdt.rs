use crate::font::Font;
use crate::util::Buffer;

/// ## `EBDT` &mdash; Embedded Bitmap Data Table
///
/// Specification: <https://docs.microsoft.com/en-us/typography/opentype/spec/ebdt>.
///
/// The `EBDT` table is used to embed monochrome or grayscale bitmap glyph
/// data. It is used together with the `EBLC` table, which provides embedded
/// bitmap locators, and the `EBSC` table, which provides embedded bitmap
/// scaling information.

#[allow(non_camel_case_types)]
#[derive(Debug)]
pub struct Table_EBDT {
}

impl Font {
    #[allow(non_snake_case)]
    pub fn parse_EBDT(&mut self, _buffer: &mut Buffer) {
    }
}
