use crate::font::{Font, TableRecord};
use crate::util::{Buffer, FWord, Fixed};

/// ## `post` &mdash; PostScript Table
///
/// <https://docs.microsoft.com/en-us/typography/opentype/spec/post>
///
/// This table contains additional information needed to use TrueType or
/// OpenType fonts on PostScript printers. This includes data for the FontInfo
/// dictionary entry and the PostScript names of all the glyphs.

#[allow(non_camel_case_types)]
#[derive(Debug)]
pub struct Table_post {
    _version: Fixed,
    pub italic_angle: Fixed,
    pub underline_position: FWord,
    pub underline_thickness: FWord,
    pub is_fixed_pitch: u32,
    pub min_mem_type42: u32,
    pub max_mem_type42: u32,
    pub min_mem_type1: u32,
    pub max_mem_type1: u32,
    // Version 2.0 and 2.5
    pub num_glyphs: Option<u16>,
    // Version 2.0
    pub glyph_name_index: Option<Vec<u16>>,
    pub names: Option<Vec<i8>>,
    // Version 2.5 (deprecated)
    pub offset: Option<Vec<i8>>,
}

impl Font {
    pub fn parse_post(&mut self, buffer: &mut Buffer, record: &TableRecord) {
        buffer.offset = record.offset as usize;
        let mut table = Table_post {
            _version: buffer.get::<Fixed>(),
            italic_angle: buffer.get::<Fixed>(),
            underline_position: buffer.get::<FWord>(),
            underline_thickness: buffer.get::<FWord>(),
            is_fixed_pitch: buffer.get::<u32>(),
            min_mem_type42: buffer.get::<u32>(),
            max_mem_type42: buffer.get::<u32>(),
            min_mem_type1: buffer.get::<u32>(),
            max_mem_type1: buffer.get::<u32>(),
            num_glyphs: None,
            glyph_name_index: None,
            names: None,
            offset: None,
        };
        if table._version == 0x0002_0000 {
            table.num_glyphs = Some(buffer.get::<u16>());
            let num_glyphs = table.num_glyphs.unwrap() as usize;
            table.glyph_name_index = Some(buffer.get_vec::<u16>(num_glyphs));
            table.names = Some(buffer.get_vec::<i8>(num_glyphs));
        }
        if table._version == 0x0002_5000 {
            table.num_glyphs = Some(buffer.get::<u16>());
            let num_glyphs = table.num_glyphs.unwrap() as usize;
            table.offset = Some(buffer.get_vec::<i8>(num_glyphs));
        }
        self.post = Some(table);
    }
}
