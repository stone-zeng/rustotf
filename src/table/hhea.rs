use crate::font::{FontTable, TableRecord};
use crate::util;
use std::mem::size_of;

#[allow(non_camel_case_types)]
#[derive(Debug)]
pub struct Table_hhea {
    version: String,
    ascender: i16,
    descender: i16,
    line_gap: i16,
    advance_width_max: u16,
    min_left_side_bearing: i16,
    min_right_side_bearing: i16,
    x_max_extent: i16,
    caret_slope_rise: i16,
    caret_slope_run: i16,
    caret_offset: i16,
    metric_data_format: i16,
    number_of_h_metrics: u16,
}

impl FontTable for Table_hhea {
    fn parse(buffer: &mut util::Buffer, record: &TableRecord) -> Self {
        buffer.offset = record.offset;
        Self {
            version: util::get_version_string(buffer.read_u16(), buffer.read_u16()),
            ascender: buffer.read_i16(),
            descender: buffer.read_i16(),
            line_gap: buffer.read_i16(),
            advance_width_max: buffer.read_u16(),
            min_left_side_bearing: buffer.read_i16(),
            min_right_side_bearing: buffer.read_i16(),
            x_max_extent: buffer.read_i16(),
            caret_slope_rise: buffer.read_i16(),
            caret_slope_run: buffer.read_i16(),
            caret_offset: buffer.read_i16(),
            metric_data_format: {
                buffer.skip(4 * size_of::<i16>() as u32);
                buffer.read_i16()
            },
            number_of_h_metrics: buffer.read_u16(),
        }
    }
}
