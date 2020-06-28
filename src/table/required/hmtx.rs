use crate::font::Font;
use crate::util::{Buffer, ReadBuffer};

/// ## `hmtx` &mdash; Horizontal Metrics Table
///
/// Specification: <https://docs.microsoft.com/en-us/typography/opentype/spec/hmtx>.
///
/// Glyph metrics used for horizontal text layout include glyph advance widths,
/// side bearings and X-direction min and max values (`xMin`, `xMax`). These are
/// derived using a combination of the glyph outline data (`glyf`, `CFF ` or
/// `CFF2`) and the horizontal metrics table. The horizontal metrics (`hmtx`)
/// table provides glyph advance widths and left side bearings.

#[allow(non_camel_case_types)]
#[derive(Debug)]
pub struct Table_hmtx {
    pub hor_metrics: Vec<LongHorMetric>,
    pub left_side_bearings: Vec<i16>,
}

impl Font {
    pub fn parse_hmtx(&mut self, buffer: &mut Buffer) {
        let num_hor_metrics = self.hhea.as_ref().unwrap().num_hor_metrics as usize;
        let num_glyphs = self.maxp.as_ref().unwrap().num_glyphs as usize;
        self.hmtx = Some(Table_hmtx {
            hor_metrics: buffer.get_vec::<LongHorMetric>(num_hor_metrics),
            left_side_bearings: buffer.get_vec::<i16>(num_glyphs),
        });
    }
}

#[derive(Debug)]
pub struct LongHorMetric {
    advance_width: u16,
    left_side_bearing: i16,
}

impl ReadBuffer for LongHorMetric {
    fn read(buffer: &mut Buffer) -> Self {
        Self {
            advance_width: buffer.get::<u16>(),
            left_side_bearing: buffer.get::<i16>(),
        }
    }
}
