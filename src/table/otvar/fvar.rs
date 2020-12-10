use crate::font::Font;
use crate::util::{Buffer, Fixed, Tag, ReadBuffer};

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
        let _version = buffer.get_version::<u16>();
        let axes_array_offset = buffer.get();
        let _axis_count = {
            buffer.skip::<u16>(1);
            buffer.get()
        };
        let _axis_size = buffer.get();
        let _instance_count = buffer.get();
        let _instance_size = buffer.get();
        let _axes = buffer.get_vec(_axis_count as usize);
        let _instances = (0.._instance_count)
            .map(|_| Instance::read(buffer, _axis_count as usize))
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
            axis_tag: buffer.get(),
            min_value: buffer.get(),
            default_value: buffer.get(),
            max_value: buffer.get(),
            flags: buffer.get(),
            axis_name_id: buffer.get(),
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
impl Instance {
    fn read(buffer: &mut Buffer, axis_count: usize) -> Self {
        Self {
            subfamily_name_id: buffer.get(),
            flags: buffer.get(),
            coordinates: buffer.get_vec(axis_count),
            postscript_name_id: buffer.get(),
        }
    }
}
