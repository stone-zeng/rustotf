use crate::font::Font;
use crate::util::Buffer;

/// ## `EBSC` &mdash; Embedded Bitmap Scaling Table
///
/// Specification: <https://docs.microsoft.com/en-us/typography/opentype/spec/ebsc>.
///
/// The `EBSC` table provides a mechanism for describing embedded bitmaps which
/// are created by scaling other embedded bitmaps. While this is the sort of
/// thing that outline font technologies were invented to avoid, there are
/// cases (small sizes of Kanji, for example) where scaling a bitmap produces
/// a more legible font than scan-converting an outline. For this reason the
/// `EBSC` table allows a font to define a bitmap strike as a scaled version
/// of another strike.

#[allow(non_camel_case_types)]
#[derive(Debug)]
pub struct Table_EBSC {
}

impl Font {
    #[allow(non_snake_case)]
    pub fn parse_EBSC(&mut self, _buffer: &mut Buffer) {
    }
}
