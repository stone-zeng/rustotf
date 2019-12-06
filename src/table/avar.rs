use crate::font::Font;
use crate::util::{get_version_string, Buffer, F2Dot14, ReadBuffer};

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
        let _version = get_version_string(buffer.get::<u16>(), buffer.get::<u16>());
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

// #[derive(Debug)]
// struct Instance {
//     pub subfamily_name_id: u16,
//     pub flags: u16,
//     pub coordinates: Vec<Fixed>,
//     pub postscript_name_id: u16,
// }

// // We can't use trait `ReadBuffer` here because reading `Instance` requires
// // `axis_count`, which from the outside structure.
// fn read_instance(buffer: &mut Buffer, axis_count: usize) -> Instance {
//     let subfamily_name_id = buffer.get::<u16>();
//     let flags = buffer.get::<u16>();
//     let coordinates = buffer.get_vec::<Fixed>(axis_count);
//     let postscript_name_id = buffer.get::<u16>();
//     Instance {
//         subfamily_name_id,
//         flags,
//         coordinates,
//         postscript_name_id,
//     }
// }
