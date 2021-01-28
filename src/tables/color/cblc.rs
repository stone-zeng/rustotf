use crate::font::Font;
use crate::tables::bitmap::eblc::Strike;
use crate::util::Buffer;

/// ## `CBLC` &mdash; Color Bitmap Location Table
///
/// Specification: <https://docs.microsoft.com/en-us/typography/opentype/spec/cblc>.
///
/// The `CBLC` table provides embedded bitmap locators. It is used together with the `CBDT`
/// table, which provides embedded, color bitmap glyph data. The formats of these two tables
/// are backward compatible with the `EBDT` and `EBLC` tables used for embedded monochrome
/// and grayscale bitmaps.

#[allow(non_camel_case_types)]
#[derive(Debug)]
pub struct Table_CBLC {
    version: String,
    num_strikes: u32,
    pub strikes: Vec<Strike>,
}

impl Font {
    #[allow(non_snake_case)]
    pub fn parse_CBLC(&mut self, buffer: &mut Buffer) {
        let cblc_start = buffer.offset();
        let version = buffer.get_version::<u16>();
        let num_strikes = buffer.get();
        let strikes = Strike::read_vec(buffer, num_strikes as usize, cblc_start);
        self.CBLC = Some(Table_CBLC {
            version,
            num_strikes,
            strikes,
        })
    }
}
