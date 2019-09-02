use crate::font::{Font, TableRecord};
use crate::util;

/// ## `hmtx` &mdash; Horizontal Metrics Table
///
/// <https://docs.microsoft.com/en-us/typography/opentype/spec/hmtx>
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
    pub fn parse_hmtx(&mut self, buffer: &mut util::Buffer, record: &TableRecord) {
        buffer.offset = record.offset;
        let num_hor_metrics = self.hhea.as_ref().unwrap().num_hor_metrics;
        let num_glyphs = self.maxp.as_ref().unwrap().num_glyphs;
        let mut hor_metrics: Vec<LongHorMetric> = Vec::new();
        for _ in 0..num_hor_metrics {
            let hor_metric = LongHorMetric {
                advance_width: buffer.read::<u16>(),
                left_side_bearing: buffer.read::<i16>(),
            };
            hor_metrics.push(hor_metric);
        }
        let mut left_side_bearings: Vec<i16> = Vec::new();
        for _ in 0..(num_glyphs - num_hor_metrics) {
            let left_side_bearing = buffer.read::<i16>();
            left_side_bearings.push(left_side_bearing);
        }
        self.hmtx = Some(Table_hmtx {
            hor_metrics,
            left_side_bearings,
        });
    }
}

#[derive(Debug)]
pub struct LongHorMetric {
    advance_width: u16,
    left_side_bearing: i16,
}
