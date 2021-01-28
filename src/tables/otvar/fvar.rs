use crate::font::Font;
use crate::util::{Buffer, Fixed, ReadBuffer, Tag};
use read_buffer_derive::ReadBuffer;

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
    version: String,
    axes_array_offset: u16,
    // Reserved `uint16` here.
    axis_count: u16,
    axis_size: u16,
    instance_count: u16,
    instance_size: u16,
    axes: Vec<VariationAxis>,
    instances: Vec<Instance>,
}

impl Font {
    pub fn parse_fvar(&mut self, buffer: &mut Buffer) {
        let version = buffer.get_version::<u16>();
        let axes_array_offset = buffer.get();
        let axis_count = {
            buffer.skip::<u16>(1);
            buffer.get()
        };
        let axis_size = buffer.get();
        let instance_count = buffer.get();
        let instance_size = buffer.get();
        let axes = buffer.get_vec(axis_count);
        let instances = (0..instance_count)
            .map(|_| Instance::read(buffer, axis_count as usize))
            .collect();

        self.fvar = Some(Table_fvar {
            version,
            axes_array_offset,
            axis_count,
            axis_size,
            instance_count,
            instance_size,
            axes,
            instances,
        });
    }
}

#[derive(Debug, ReadBuffer)]
struct VariationAxis {
    pub axis_tag: Tag,
    pub min_value: Fixed,
    pub default_value: Fixed,
    pub max_value: Fixed,
    pub flags: u16,
    pub axis_name_id: u16,
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
