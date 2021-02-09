use crate::font::Font;
use crate::types::F2Dot14;
use crate::util::{Buffer, ReadBuffer};
use read_buffer_derive::ReadBuffer;

/// ## `avar` &mdash; Axis Variations Table
///
/// Specification: <https://docs.microsoft.com/en-us/typography/opentype/spec/avar>.
///
/// The axis variations table (`avar`) is an optional table used in variable
/// fonts that use OpenType Font Variations mechanisms. It can be used to
/// modify aspects of how a design varies for different instances along a
/// particular design-variation axis. Specifically, it allows modification of
/// the coordinate normalization that is used when processing variation data
/// for a particular variation instance.

#[allow(non_camel_case_types)]
#[derive(Debug)]
pub struct Table_avar {
    version: String,
    // Reserved `uint16` here.
    axis_count: u16,
    axis_segment_maps: Vec<SegmentMaps>,
}

impl Font {
    pub fn parse_avar(&mut self, buffer: &mut Buffer) {
        let version = buffer.get_version::<u16>();
        let axis_count = {
            buffer.skip::<u16>(1);
            buffer.get()
        };
        let axis_segment_maps = buffer.get_vec(axis_count);

        self.avar = Some(Table_avar {
            version,
            axis_count,
            axis_segment_maps,
        });
    }
}

#[derive(Debug)]
struct SegmentMaps {
    position_map_count: u16,
    axis_value_maps: Vec<AxisValueMap>,
}

impl ReadBuffer for SegmentMaps {
    fn read(buffer: &mut Buffer) -> Self {
        let position_map_count = buffer.get();
        let axis_value_maps = buffer.get_vec(position_map_count);
        Self {
            position_map_count,
            axis_value_maps,
        }
    }
}

#[derive(Debug, ReadBuffer)]
struct AxisValueMap {
    pub from_coordinate: F2Dot14,
    pub to_coordinate: F2Dot14,
}
