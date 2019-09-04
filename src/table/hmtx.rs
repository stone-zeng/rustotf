use crate::font::{Font, TableRecord};
use crate::util::{Buffer, ReadFromBuffer, U16_SIZE};

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
    pub fn parse_hmtx(&mut self, buffer: &mut Buffer, record: &TableRecord) {
        buffer.offset = record.offset;
        let num_hor_metrics = self.hhea.as_ref().unwrap().num_hor_metrics as usize;
        let num_glyphs = self.maxp.as_ref().unwrap().num_glyphs as usize;
        self.hmtx = Some(Table_hmtx {
            hor_metrics: buffer.read_vec::<LongHorMetric>(num_hor_metrics),
            left_side_bearings: buffer.read_vec::<i16>(num_glyphs),
        });
    }
}

#[derive(Debug)]
pub struct LongHorMetric {
    advance_width: u16,
    left_side_bearing: i16,
}

impl ReadFromBuffer for LongHorMetric {
    fn read_from_buffer(_buffer: &Vec<u8>, _offset: usize) -> Self {
        Self {
            advance_width: u16::read_from_buffer(_buffer, _offset),
            left_side_bearing: i16::read_from_buffer(_buffer, _offset + U16_SIZE),
        }
    }
}
