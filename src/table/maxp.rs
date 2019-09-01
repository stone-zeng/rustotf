use crate::font::{FontTable, TableRecord};
use crate::util;

#[allow(non_camel_case_types)]
#[derive(Debug)]
pub struct Table_maxp {
    version: util::Fixed,
    num_glyphs: u16,
    max_points: Option<u16>,
    max_contours: Option<u16>,
    max_composite_points: Option<u16>,
    max_composite_contours: Option<u16>,
    max_zones: Option<u16>,
    max_twilight_points: Option<u16>,
    max_storage: Option<u16>,
    max_function_defs: Option<u16>,
    max_instruction_defs: Option<u16>,
    max_stack_elements: Option<u16>,
    max_size_of_instructions: Option<u16>,
    max_component_elements: Option<u16>,
    max_component_depth: Option<u16>,
}

impl FontTable for Table_maxp {
    fn parse(buffer: &mut util::Buffer, record: &TableRecord) -> Self {
        buffer.offset = record.offset;
        let mut table = Self {
            version: buffer.read_fixed(),
            num_glyphs: buffer.read_u16(),
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
        if table.version == 0x00010000 {
            table.max_points = Some(buffer.read_u16());
            table.max_contours = Some(buffer.read_u16());
            table.max_composite_points = Some(buffer.read_u16());
            table.max_composite_contours = Some(buffer.read_u16());
            table.max_zones = Some(buffer.read_u16());
            table.max_twilight_points = Some(buffer.read_u16());
            table.max_storage = Some(buffer.read_u16());
            table.max_function_defs = Some(buffer.read_u16());
            table.max_instruction_defs = Some(buffer.read_u16());
            table.max_stack_elements = Some(buffer.read_u16());
            table.max_size_of_instructions = Some(buffer.read_u16());
            table.max_component_elements = Some(buffer.read_u16());
            table.max_component_depth = Some(buffer.read_u16());
        }

        table
    }
}
