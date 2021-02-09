use crate::font::Font;
use crate::types::Tag;
use crate::util::{Buffer, ReadBuffer};
use read_buffer_derive::ReadBuffer;

/// ## `MVAR` &mdash; Metrics Variations Table
///
/// Specification: <https://docs.microsoft.com/en-us/typography/opentype/spec/mvar>.
///
/// The metrics variations table is used in variable fonts to provide
/// variations for font-wide metric values found in the `OS/2` table and other
/// font tables. For a general overview of OpenType Font Variation and
/// terminology related to variations, see the chapter,
/// [OpenType Font Variations Overview](https://docs.microsoft.com/en-us/typography/opentype/spec/otvaroverview).

#[allow(non_camel_case_types)]
#[derive(Debug)]
pub struct Table_MVAR {
    version: String,
    // Reserved `uint16` here.
    value_record_size: u16,
    value_record_count: u16,
    item_variation_store_offset: u16,
    value_records: Vec<Value>,
}

impl Font {
    #[allow(non_snake_case)]
    pub fn parse_MVAR(&mut self, buffer: &mut Buffer) {
        let version = buffer.get_version::<u16>();
        let value_record_size = {
            buffer.skip::<u16>(1);
            buffer.get()
        };
        let value_record_count = buffer.get();
        let item_variation_store_offset = buffer.get();
        let value_records = buffer.get_vec(value_record_count);

        self.MVAR = Some(Table_MVAR {
            version,
            value_record_size,
            value_record_count,
            item_variation_store_offset,
            value_records,
        });
    }
}

#[derive(Debug, ReadBuffer)]
struct Value {
    pub value_tag: Tag,
    pub delta_set_outer_index: u16,
    pub delta_set_inner_index: u16,
}
