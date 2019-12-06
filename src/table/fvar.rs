use crate::font::Font;
use crate::util::{get_version_string, Buffer, Fixed, Tag, ReadBuffer};

/// ## `fvar` &mdash; Font Variations Table
///
/// Specification: <https://docs.microsoft.com/en-us/typography/opentype/spec/fvar>.
///
/// OpenType Font Variations allow a font designer to incorporate multiple
/// faces within a font family into a single font resource. Variable fonts can
/// provide great flexibility for content authors and designers while also
/// allowing the font data to be represented in an efficient format.

#[allow(non_camel_case_types)]
#[derive(Debug)]
pub struct Table_fvar {
    _version: String,
    axes_array_offset: u16,
    // Reserved `uint16` here.
    _axis_count: u16,
    _axis_size: u16,
    _instance_count: u16,
    _instance_size: u16,
    _axes: Vec<VariationAxis>,
    _instances: Vec<Instance>,
}

impl Font {
    pub fn parse_fvar(&mut self, buffer: &mut Buffer) {
        let _version = get_version_string(buffer.get::<u16>(), buffer.get::<u16>());
        let axes_array_offset = buffer.get::<u16>();
        buffer.skip::<u16>(1);
        let _axis_count = buffer.get::<u16>();
        let _axis_size = buffer.get::<u16>();
        let _instance_count = buffer.get::<u16>();
        let _instance_size = buffer.get::<u16>();
        let _axes = buffer.get_vec::<VariationAxis>(_axis_count as usize);
        let _instances = (0.._instance_count)
            .map(|_| read_instance(buffer, _axis_count as usize))
            .collect();

        self.fvar = Some(Table_fvar {
            _version,
            axes_array_offset,
            _axis_count,
            _axis_size,
            _instance_count,
            _instance_size,
            _axes,
            _instances,
        });
    }
}

#[derive(Debug)]
struct VariationAxis {
    pub axis_tag: Tag,
    pub min_value: Fixed,
    pub default_value: Fixed,
    pub max_value: Fixed,
    pub flags: u16,
    pub axis_name_id: u16,
}

impl ReadBuffer for VariationAxis {
    fn read(buffer: &mut Buffer) -> Self {
        Self {
            axis_tag: buffer.get::<Tag>(),
            min_value: buffer.get::<Fixed>(),
            default_value: buffer.get::<Fixed>(),
            max_value: buffer.get::<Fixed>(),
            flags: buffer.get::<u16>(),
            axis_name_id: buffer.get::<u16>(),
        }
    }
}

#[derive(Debug)]
struct Instance {
    pub subfamily_name_id: u16,
    pub flags: u16,
    pub coordinates: Vec<Fixed>,
    pub postscript_name_id: u16,
}

// We can't use trait `ReadBuffer` here because reading `Instance` requires
// `axis_count`, which from the outside structure.
fn read_instance(buffer: &mut Buffer, axis_count: usize) -> Instance {
    let subfamily_name_id = buffer.get::<u16>();
    let flags = buffer.get::<u16>();
    let coordinates = buffer.get_vec::<Fixed>(axis_count);
    let postscript_name_id = buffer.get::<u16>();
    Instance {
        subfamily_name_id,
        flags,
        coordinates,
        postscript_name_id,
    }
}
