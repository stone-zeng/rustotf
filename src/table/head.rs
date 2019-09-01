use crate::font::{FontTable, TableRecord};
use crate::util;

use chrono::NaiveDateTime;

#[allow(non_camel_case_types)]
#[derive(Debug)]
pub struct Table_head {
    version: String,
    font_revision: util::Fixed,
    check_sum_adjustment: u32,
    magic_number: u32,
    flags: u16,
    units_per_em: u16,
    created: NaiveDateTime,
    modified: NaiveDateTime,
    x_min: i16,
    y_min: i16,
    x_max: i16,
    y_max: i16,
    mac_style: u16,
    lowest_rec_ppem: u16,
    font_direction_hint: i16,
    index_to_loc_format: i16,
    glyph_data_format: i16,
}

impl FontTable for Table_head {
    fn parse(buffer: &mut util::Buffer, record: &TableRecord) -> Self {
        buffer.offset = record.offset;
        Self {
            version: util::get_version_string(buffer.read_u16(), buffer.read_u16()),
            font_revision: buffer.read_fixed(),
            check_sum_adjustment: buffer.read_u32(),
            magic_number: buffer.read_u32(),
            flags: buffer.read_u16(),
            units_per_em: buffer.read_u16(),
            created: buffer.read_datetime(),
            modified: buffer.read_datetime(),
            x_min: buffer.read_i16(),
            y_min: buffer.read_i16(),
            x_max: buffer.read_i16(),
            y_max: buffer.read_i16(),
            mac_style: buffer.read_u16(),
            lowest_rec_ppem: buffer.read_u16(),
            font_direction_hint: buffer.read_i16(),
            index_to_loc_format: buffer.read_i16(),
            glyph_data_format: buffer.read_i16(),
        }
    }
}
