use crate::font::Font;
use crate::util::{Buffer};

/// ## `loca` &mdash; Index to Location
///
/// Specification: <https://docs.microsoft.com/en-us/typography/opentype/spec/loca>.
///
/// The indexToLoc table stores the offsets to the locations of the glyphs in the font,
/// relative to the beginning of the glyphData table. In order to compute the length of
/// the last glyph element, there is an extra entry after the last valid index.

#[allow(non_camel_case_types)]
#[derive(Debug)]
pub struct Table_loca {
    pub short_offsets: Option<Vec<u16>>,
    pub long_offsets: Option<Vec<u32>>,
}

impl Font {
    pub fn parse_loca(&mut self, buffer: &mut Buffer) {
        let mut table = Table_loca {
            short_offsets: None,
            long_offsets: None,
        };

        println!("{:?}", self.head);


        let index_to_loc_format = self.head.as_ref().unwrap().index_to_loc_format;
        let num_glyphs = self.maxp.as_ref().unwrap().num_glyphs as usize;
        match index_to_loc_format {
            0 => table.short_offsets = Some(buffer.get_vec::<u16>(num_glyphs)),
            1 => table.long_offsets = Some(buffer.get_vec::<u32>(num_glyphs)),
            _ => (),
        }
        self.loca = Some(table);
    }
}
