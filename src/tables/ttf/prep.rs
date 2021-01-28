use std::mem::size_of;

use crate::font::Font;
use crate::util::{Buffer, Tag};

/// ## `prep` &mdash; Control Value Program
///
/// Specification: <https://docs.microsoft.com/en-us/typography/opentype/spec/prep>.
///
/// The Control Value Program consists of a set of TrueType instructions that will be executed
/// whenever the font or point size or transformation matrix change and before each glyph is
/// interpreted. Any instruction is legal in the CV Program but since no glyph is associated
/// with it, instructions intended to move points within a particular glyph outline cannot be used
/// in the CV Program. The name `prep` is anachronistic (the table used to be known as the
/// Pre Program table.)

#[allow(non_camel_case_types)]
#[derive(Debug)]
pub struct Table_prep {
    values: Vec<u8>,
}

impl Font {
    pub fn parse_prep(&mut self, buffer: &mut Buffer) {
        let num = self.get_table_len(Tag::new(b"prep")) / size_of::<u8>();
        self.prep = Some(Table_prep {
            values: buffer.get_vec(num),
        });
    }
}
