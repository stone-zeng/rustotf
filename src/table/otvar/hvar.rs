use crate::font::Font;
use crate::util::Buffer;

/// ## `HVAR` &mdash; Horizontal Metrics Variations Table
///
/// Specification: <https://docs.microsoft.com/zh-cn/typography/opentype/spec/hvar>.
///
/// The `HVAR` table is used in variable fonts to provide variations for
/// horizontal glyph metrics values. This can be used to provide variation data
/// for advance widths in the `hmtx` table. In fonts with TrueType outlines,
/// it can also be used to provide variation data for left and right side
/// bearings obtained from the `hmtx` table and glyph bounding box.

#[allow(non_camel_case_types)]
#[derive(Debug)]
pub struct Table_HVAR {
    _version: String,
    _item_variation_store_offset: u32,
    _advance_width_mapping_offset: u32,
    _lsb_mapping_offset: u32,
    _rsb_mapping_offset: u32,
}

impl Font {
    #[allow(non_snake_case)]
    pub fn parse_HVAR(&mut self, buffer: &mut Buffer) {
        self.HVAR = Some(Table_HVAR {
            _version: buffer.get_version::<u16>(),
            _item_variation_store_offset: buffer.get::<u32>(),
            _advance_width_mapping_offset: buffer.get::<u32>(),
            _lsb_mapping_offset: buffer.get::<u32>(),
            _rsb_mapping_offset: buffer.get::<u32>(),
        });
    }
}
