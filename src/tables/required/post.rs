use crate::font::Font;
use crate::types::Fixed;
use crate::util::Buffer;

/// ## `post` &mdash; PostScript Table
///
/// Specification: <https://docs.microsoft.com/en-us/typography/opentype/spec/post>.
///
/// This table contains additional information needed to use TrueType or
/// OpenType fonts on PostScript printers. This includes data for the FontInfo
/// dictionary entry and the PostScript names of all the glyphs.

#[allow(non_camel_case_types)]
#[derive(Debug, Default)]
pub struct Table_post {
    version: Fixed,
    pub italic_angle: Fixed,
    pub underline_position: i16,
    pub underline_thickness: i16,
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
    pub fn parse_post(&mut self, buffer: &mut Buffer) {
        let mut table = Table_post {
            version: buffer.get(),
            italic_angle: buffer.get(),
            underline_position: buffer.get(),
            underline_thickness: buffer.get(),
            is_fixed_pitch: buffer.get(),
            min_mem_type42: buffer.get(),
            max_mem_type42: buffer.get(),
            min_mem_type1: buffer.get(),
            max_mem_type1: buffer.get(),
            ..Default::default()
        };
        if table.version == 0x0002_0000 {
            let num_glyphs = buffer.get();
            table.num_glyphs = Some(num_glyphs);
            table.glyph_name_index = Some(buffer.get_vec(num_glyphs));
            table.names = Some(buffer.get_vec(num_glyphs));
        }
        if table.version == 0x0002_5000 {
            let num_glyphs = buffer.get();
            table.num_glyphs = Some(num_glyphs);
            table.offset = Some(buffer.get_vec(num_glyphs));
        }
        self.post = Some(table);
    }
}
