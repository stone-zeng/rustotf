use crate::font::Font;
use crate::util::Buffer;

/// ## `hhea` &mdash; Horizontal Header Table
///
/// Specification: <https://docs.microsoft.com/en-us/typography/opentype/spec/hhea>.
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
    pub ascender: i16,
    pub descender: i16,
    pub line_gap: i16,
    pub advance_width_max: u16,
    pub min_left_side_bearing: i16,
    pub min_right_side_bearing: i16,
    pub x_max_extent: i16,
    pub caret_slope_rise: i16,
    pub caret_slope_run: i16,
    pub caret_offset: i16,
    // Reserved 4 * `int16` here.
    pub metric_data_format: i16,
    pub num_hor_metrics: u16,
}

impl Font {
    pub fn parse_hhea(&mut self, buffer: &mut Buffer) {
        self.hhea = Some(Table_hhea {
            _version: buffer.get_version(),
            ascender: buffer.get::<i16>(),
            descender: buffer.get::<i16>(),
            line_gap: buffer.get::<i16>(),
            advance_width_max: buffer.get::<u16>(),
            min_left_side_bearing: buffer.get::<i16>(),
            min_right_side_bearing: buffer.get::<i16>(),
            x_max_extent: buffer.get::<i16>(),
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
