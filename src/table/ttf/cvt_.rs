use std::mem::size_of;
use crate::font::Font;
use crate::util::Buffer;

/// ## `cvt` &mdash; Control Value Table
///
/// Specification: <https://docs.microsoft.com/en-us/typography/opentype/spec/cvt>.
///
/// This table contains a list of values that can be referenced by instructions.
/// They can be used, among other things, to control characteristics for different glyphs.
/// The length of the table must be an integral number of `FWORD` units.

#[allow(non_camel_case_types)]
#[derive(Debug)]
pub struct Table_cvt_ {
    values: Vec<i16>,
}

impl Font {
    pub fn parse_cvt_(&mut self, buffer: &mut Buffer) {
        self.cvt_ = Some(Table_cvt_ {
            values: buffer.get_vec(self.get_table_len("cvt ") / size_of::<i16>()),
        });
    }
}
