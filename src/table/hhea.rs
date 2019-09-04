use crate::font::{Font, TableRecord};
use crate::util::{get_version_string, Buffer, FWord, UFWord};

/// ## hhea &mdash; Horizontal Header Table
///
/// <https://docs.microsoft.com/en-us/typography/opentype/spec/hhea>
///
/// This table contains information for horizontal layout. The values in the
/// `minRightSidebearing`, `minLeftSideBearing` and `xMaxExtent` should be
/// computed using *only* glyphs that have contours. Glyphs with no contours
/// should be ignored for the purposes of these calculations. All reserved
/// areas must be set to 0.

#[allow(non_camel_case_types)]
#[derive(Debug)]
pub struct Table_hhea {
    _version: String,
    pub ascender: FWord,
    pub descender: FWord,
    pub line_gap: FWord,
    pub advance_width_max: UFWord,
    pub min_left_side_bearing: FWord,
    pub min_right_side_bearing: FWord,
    pub x_max_extent: FWord,
    pub caret_slope_rise: i16,
    pub caret_slope_run: i16,
    pub caret_offset: i16,
    // Reserved 4 * `int16` here.
    pub metric_data_format: i16,
    pub num_hor_metrics: u16,
}

impl Font {
    pub fn parse_hhea(&mut self, buffer: &mut Buffer, record: &TableRecord) {
        buffer.offset = record.offset as usize;
        self.hhea = Some(Table_hhea {
            _version: get_version_string(buffer.get::<u16>(), buffer.get::<u16>()),
            ascender: buffer.get::<FWord>(),
            descender: buffer.get::<FWord>(),
            line_gap: buffer.get::<FWord>(),
            advance_width_max: buffer.get::<UFWord>(),
            min_left_side_bearing: buffer.get::<FWord>(),
            min_right_side_bearing: buffer.get::<FWord>(),
            x_max_extent: buffer.get::<FWord>(),
            caret_slope_rise: buffer.get::<i16>(),
            caret_slope_run: buffer.get::<i16>(),
            caret_offset: buffer.get::<i16>(),
            metric_data_format: {
                buffer.skip::<i16>(4);
                buffer.get::<i16>()
            },
            num_hor_metrics: buffer.get::<u16>(),
        });
    }
}
