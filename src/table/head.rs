use crate::font::{Font, TableRecord};
use crate::util::{get_version_string, Buffer, Fixed, LongDateTime};

/// ## `head` &mdash; Font Header Table
///
/// <https://docs.microsoft.com/en-us/typography/opentype/spec/head>
///
/// This table gives global information about the font. The bounding box values
/// should be computed using *only* glyphs that have contours. Glyphs with no
/// contours should be ignored for the purposes of these calculations.

#[allow(non_camel_case_types)]
#[derive(Debug)]
pub struct Table_head {
    _version: String,
    pub font_revision: Fixed,
    pub check_sum_adjustment: u32,
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
    pub fn parse_head(&mut self, buffer: &mut Buffer, record: &TableRecord) {
        buffer.offset = record.offset;
        self.head = Some(Table_head {
            _version: get_version_string(buffer.read::<u16>(), buffer.read::<u16>()),
            font_revision: buffer.read::<Fixed>(),
            check_sum_adjustment: buffer.read::<u32>(),
            magic_number: buffer.read::<u32>(),
            flags: buffer.read::<u16>(),
            units_per_em: buffer.read::<u16>(),
            created: buffer.read::<LongDateTime>(),
            modified: buffer.read::<LongDateTime>(),
            x_min: buffer.read::<i16>(),
            y_min: buffer.read::<i16>(),
            x_max: buffer.read::<i16>(),
            y_max: buffer.read::<i16>(),
            mac_style: buffer.read::<u16>(),
            lowest_rec_ppem: buffer.read::<u16>(),
            font_direction_hint: buffer.read::<i16>(),
            index_to_loc_format: buffer.read::<i16>(),
            glyph_data_format: buffer.read::<i16>(),
        });
    }
}
