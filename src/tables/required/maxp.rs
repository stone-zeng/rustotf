use crate::font::Font;
use crate::types::Fixed;
use crate::util::Buffer;

/// ## `maxp` &mdash; Maximum Profile
///
/// Specification: <https://docs.microsoft.com/en-us/typography/opentype/spec/maxp>.
///
/// This table establishes the memory requirements for this font. Fonts with
/// CFF data must use Version 0.5 of this table, specifying only the `numGlyphs`
///  field. Fonts with TrueType outlines must use Version 1.0 of this table,
/// where all data is required.

#[allow(non_camel_case_types)]
#[derive(Debug, Default)]
pub struct Table_maxp {
    version: Fixed,
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
    pub fn parse_maxp(&mut self, buffer: &mut Buffer) {
        let mut table = Table_maxp {
            version: buffer.get(),
            num_glyphs: buffer.get(),
            ..Default::default()
        };
        // Version 1.0
        if table.version == 0x0001_0000 {
            table.max_points = Some(buffer.get());
            table.max_contours = Some(buffer.get());
            table.max_composite_points = Some(buffer.get());
            table.max_composite_contours = Some(buffer.get());
            table.max_zones = Some(buffer.get());
            table.max_twilight_points = Some(buffer.get());
            table.max_storage = Some(buffer.get());
            table.max_function_defs = Some(buffer.get());
            table.max_instruction_defs = Some(buffer.get());
            table.max_stack_elements = Some(buffer.get());
            table.max_size_of_instructions = Some(buffer.get());
            table.max_component_elements = Some(buffer.get());
            table.max_component_depth = Some(buffer.get());
        }
        self.maxp = Some(table);
    }
}
