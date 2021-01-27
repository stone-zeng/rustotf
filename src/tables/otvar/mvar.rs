use crate::font::Font;
use crate::util::{Buffer, ReadBuffer, Tag};
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
    _version: String,
    // Reserved `uint16` here.
    _value_record_size: u16,
    _value_record_count: u16,
    _item_variation_store_offset: u16,
    _value_records: Vec<Value>,
}

impl Font {
    #[allow(non_snake_case)]
    pub fn parse_MVAR(&mut self, buffer: &mut Buffer) {
        let _version = buffer.get_version::<u16>();
        let _value_record_size = {
            buffer.skip::<u16>(1);
            buffer.get()
        };
        let _value_record_count = buffer.get();
        let _item_variation_store_offset = buffer.get();
        let _value_records = buffer.get_vec(_value_record_count);

        self.MVAR = Some(Table_MVAR {
            _version,
            _value_record_size,
            _value_record_count,
            _item_variation_store_offset,
            _value_records,
        });
    }
}

#[derive(Debug, ReadBuffer)]
struct Value {
    pub value_tag: Tag,
    pub delta_set_outer_index: u16,
    pub delta_set_inner_index: u16,
}