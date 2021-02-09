use crate::font::Font;
use crate::types::{Fixed, LongDateTime};
use crate::util::Buffer;

/// ## `head` &mdash; Font Header Table
///
/// Specification: <https://docs.microsoft.com/en-us/typography/opentype/spec/head>.
///
/// This table gives global information about the font. The bounding box values
/// should be computed using *only* glyphs that have contours. Glyphs with no
/// contours should be ignored for the purposes of these calculations.

#[allow(non_camel_case_types)]
#[derive(Debug)]
pub struct Table_head {
    version: String,
    pub font_revision: Fixed,
    pub checksum_adjustment: u32,
    pub magic_number: u32,
    pub flags: u16,
    pub units_per_em: u16,
    pub created: LongDateTime,
    pub modified: LongDateTime,
    pub x_min: i16,
    pub y_min: i16,
    pub x_max: i16,
    pub y_max: i16,
    pub mac_style: u16,
    pub lowest_rec_ppem: u16,
    pub font_direction_hint: i16,
    pub index_to_loc_format: i16,
    pub glyph_data_format: i16,
}

impl Font {
    pub fn parse_head(&mut self, buffer: &mut Buffer) {
        self.head = Some(Table_head {
            version: buffer.get_version::<u16>(),
            font_revision: buffer.get(),
            checksum_adjustment: buffer.get(),
            magic_number: buffer.get(),
            flags: buffer.get(),
            units_per_em: buffer.get(),
            created: buffer.get(),
            modified: buffer.get(),
            x_min: buffer.get(),
            y_min: buffer.get(),
            x_max: buffer.get(),
            y_max: buffer.get(),
            mac_style: buffer.get(),
            lowest_rec_ppem: buffer.get(),
            font_direction_hint: buffer.get(),
            index_to_loc_format: buffer.get(),
            glyph_data_format: buffer.get(),
        });
    }
}
