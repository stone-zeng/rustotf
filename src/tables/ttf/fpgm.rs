use std::mem::size_of;

use crate::font::Font;
use crate::types::Tag;
use crate::util::Buffer;

/// ## `fpgm` &mdash; Font Program
///
/// Specification: <https://docs.microsoft.com/en-us/typography/opentype/spec/fpgm>.
///
/// This table is similar to the CVT Program, except that it is only run once, when the font is
/// first used. It is used only for FDEFs and IDEFs. Thus the CVT Program need not contain function
/// definitions. However, the CVT Program may redefine existing FDEFs or IDEFs.

#[allow(non_camel_case_types)]
#[derive(Debug)]
pub struct Table_fpgm {
    values: Vec<u8>,
}

impl Font {
    pub fn parse_fpgm(&mut self, buffer: &mut Buffer) {
        let num = self.get_table_len(Tag::new(b"fpgm")) / size_of::<u8>();
        self.fpgm = Some(Table_fpgm {
            values: buffer.get_vec(num),
        });
    }
}
