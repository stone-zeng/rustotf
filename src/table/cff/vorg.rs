use crate::font::Font;
use crate::util::{Buffer, ReadBuffer};

use read_buffer_derive::ReadBuffer;

/// ## `VORG` &mdash; Vertical Origin Table
///
/// Specification: <https://docs.microsoft.com/en-us/typography/opentype/spec/vorg>.
///
/// This optional table specifies the y coordinate of the vertical origin of every glyph
/// in the font.

#[allow(non_camel_case_types)]
#[derive(Debug)]
pub struct Table_VORG {
    _version: String,
    pub default_vert_origin_y: i16,
    pub num_vert_origin_y_metrics: u16,
    pub vert_origin_y_metrics: Vec<VOriginRecord>,
}

impl Font {
    #[allow(non_snake_case)]
    pub fn parse_VORG(&mut self, buffer: &mut Buffer) {
        let _version = buffer.get_version::<u16>();
        let default_vert_origin_y = buffer.get();
        let num_vert_origin_y_metrics = buffer.get();
        let vert_origin_y_metrics = buffer.get_vec(num_vert_origin_y_metrics);
        self.VORG = Some(Table_VORG {
            _version,
            default_vert_origin_y,
            num_vert_origin_y_metrics,
            vert_origin_y_metrics,
        });
    }
}

#[derive(Debug, ReadBuffer)]
pub struct VOriginRecord {
    glyph_index: u16,
    vert_origin_y: i16,
}
