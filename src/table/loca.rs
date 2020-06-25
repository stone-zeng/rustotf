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
    pub offsets: Vec<usize>,
}

impl Font {
    pub fn parse_loca(&mut self, buffer: &mut Buffer) {
        let index_to_loc_format = self.head.as_ref().unwrap().index_to_loc_format;
        let num_glyphs = self.maxp.as_ref().unwrap().num_glyphs as usize;
        let mut offsets = Vec::new();
        match index_to_loc_format {
            0 => for _ in 0..num_glyphs {
                offsets.push(buffer.get::<u16>() as usize * 2)
            },
            1 => for _ in 0..num_glyphs {
                offsets.push(buffer.get::<u32>() as usize)
            },
            _ => (),
        }
        self.loca = Some(Table_loca { offsets });
    }
}
