use crate::font::Font;
use crate::util::{Buffer, F2Dot14, ReadBuffer};

/// ## `avar` &mdash; Axis Variations Table
///
/// Specification: <https://docs.microsoft.com/zh-cn/typography/opentype/spec/avar>.
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
    _version: String,
    // Reserved `uint16` here.
    _axis_count: u16,
    _axis_segment_maps: Vec<SegmentMaps>,
}

impl Font {
    pub fn parse_avar(&mut self, buffer: &mut Buffer) {
        let _version = buffer.get_version();
        buffer.skip::<u16>(1);
        let _axis_count = buffer.get::<u16>();
        let _axis_segment_maps = buffer.get_vec::<SegmentMaps>(_axis_count as usize);

        self.avar = Some(Table_avar {
            _version,
            _axis_count,
            _axis_segment_maps,
        });
    }
}

#[derive(Debug)]
struct SegmentMaps {
    _position_map_count: u16,
    _axis_value_maps: Vec<AxisValueMap>,
}

impl ReadBuffer for SegmentMaps {
    fn read(buffer: &mut Buffer) -> Self {
        let _position_map_count = buffer.get::<u16>();
        let _axis_value_maps = buffer.get_vec::<AxisValueMap>(_position_map_count as usize);
        Self {
            _position_map_count,
            _axis_value_maps,
        }
    }
}

#[derive(Debug)]
struct AxisValueMap {
    pub from_coordinate: F2Dot14,
    pub to_coordinate: F2Dot14,
}

impl ReadBuffer for AxisValueMap {
    fn read(buffer: &mut Buffer) -> Self {
        Self {
            from_coordinate: buffer.get::<F2Dot14>(),
            to_coordinate: buffer.get::<F2Dot14>(),
        }
    }
}