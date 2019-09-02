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
        buffer.offset = record.offset;
        self.hhea = Some(Table_hhea {
            _version: get_version_string(buffer.read::<u16>(), buffer.read::<u16>()),
            ascender: buffer.read::<FWord>(),
            descender: buffer.read::<FWord>(),
            line_gap: buffer.read::<FWord>(),
            advance_width_max: buffer.read::<UFWord>(),
            min_left_side_bearing: buffer.read::<FWord>(),
            min_right_side_bearing: buffer.read::<FWord>(),
            x_max_extent: buffer.read::<FWord>(),
            caret_slope_rise: buffer.read::<i16>(),
            caret_slope_run: buffer.read::<i16>(),
            caret_offset: buffer.read::<i16>(),
            metric_data_format: {
                buffer.skip::<i16>(4);
                buffer.read::<i16>()
            },
            num_hor_metrics: buffer.read::<u16>(),
        });
    }
}
