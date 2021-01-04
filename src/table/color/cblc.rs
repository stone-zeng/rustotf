use crate::font::Font;
use crate::table::bitmap::eblc::Strike;
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
    _version: String,
    _num_strikes: u32,
    pub strikes: Vec<Strike>,
}

impl Font {
    #[allow(non_snake_case)]
    pub fn parse_CBLC(&mut self, buffer: &mut Buffer) {
        let cblc_start_offset = buffer.offset;
        let _version = buffer.get_version::<u16>();
        let _num_strikes = buffer.get();
        let strikes = Strike::read_vec(buffer, _num_strikes as usize, cblc_start_offset);
        self.CBLC = Some(Table_CBLC {
            _version,
            _num_strikes,
            strikes,
        })
    }
}
