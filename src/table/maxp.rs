use crate::font::{Font, TableRecord};
use crate::util::{Buffer, Fixed};

/// ## `maxp` &mdash; Maximum Profile
///
/// <https://docs.microsoft.com/en-us/typography/opentype/spec/maxp>
///
/// This table establishes the memory requirements for this font. Fonts with
/// CFF data must use Version 0.5 of this table, specifying only the `numGlyphs`
///  field. Fonts with TrueType outlines must use Version 1.0 of this table,
/// where all data is required.

#[allow(non_camel_case_types)]
#[derive(Debug)]
pub struct Table_maxp {
    _version: Fixed,
    pub num_glyphs: u16,
    pub max_points: Option<u16>,
    pub max_contours: Option<u16>,
    pub max_composite_points: Option<u16>,
    pub max_composite_contours: Option<u16>,
    pub max_zones: Option<u16>,
    pub max_twilight_points: Option<u16>,
    pub max_storage: Option<u16>,
    pub max_function_defs: Option<u16>,
    pub max_instruction_defs: Option<u16>,
    pub max_stack_elements: Option<u16>,
    pub max_size_of_instructions: Option<u16>,
    pub max_component_elements: Option<u16>,
    pub max_component_depth: Option<u16>,
}

impl Font {
    pub fn parse_maxp(&mut self, buffer: &mut Buffer, record: &TableRecord) {
        buffer.offset = record.offset;
        let mut table = Table_maxp {
            _version: buffer.read::<Fixed>(),
            num_glyphs: buffer.read::<u16>(),
            max_points: None,
            max_contours: None,
            max_composite_points: None,
            max_composite_contours: None,
            max_zones: None,
            max_twilight_points: None,
            max_storage: None,
            max_function_defs: None,
            max_instruction_defs: None,
            max_stack_elements: None,
            max_size_of_instructions: None,
            max_component_elements: None,
            max_component_depth: None,
        };
        // Version 1.0
        if table._version == 0x00010000 {
            table.max_points = Some(buffer.read::<u16>());
            table.max_contours = Some(buffer.read::<u16>());
            table.max_composite_points = Some(buffer.read::<u16>());
            table.max_composite_contours = Some(buffer.read::<u16>());
            table.max_zones = Some(buffer.read::<u16>());
            table.max_twilight_points = Some(buffer.read::<u16>());
            table.max_storage = Some(buffer.read::<u16>());
            table.max_function_defs = Some(buffer.read::<u16>());
            table.max_instruction_defs = Some(buffer.read::<u16>());
            table.max_stack_elements = Some(buffer.read::<u16>());
            table.max_size_of_instructions = Some(buffer.read::<u16>());
            table.max_component_elements = Some(buffer.read::<u16>());
            table.max_component_depth = Some(buffer.read::<u16>());
        }
        self.maxp = Some(table);
    }
}
