use crate::font::{Font, TableRecord};
use crate::util::{u24, Buffer, Offset32, Read};

use std::collections::HashMap;

/// ## `cmap` &mdash; Character to Glyph Index Mapping Table
///
/// <https://docs.microsoft.com/en-us/typography/opentype/spec/cmap>
///
/// This table defines the mapping of character codes to the glyph index values
/// used in the font. It may contain more than one subtable, in order to support
/// more than one character encoding scheme.

#[allow(non_camel_case_types)]
#[derive(Debug)]
pub struct Table_cmap {
    _version: u16,
    pub num_tables: u16,
    pub encoding_records: Vec<EncodingRecord>,
    pub subtables: HashMap<Offset32, CmapSubtable>,

    pub data: HashMap<u32, String>,
}

impl Table_cmap {
    pub fn format(&self) {}
}

impl Font {
    pub fn parse_cmap(&mut self, buffer: &mut Buffer, record: &TableRecord) {
        buffer.offset = record.offset as usize;
        let _version = buffer.get::<u16>();
        let num_tables = buffer.get::<u16>();
        let encoding_records = buffer.get_vec::<EncodingRecord>(num_tables as usize);
        let mut subtables: HashMap<Offset32, CmapSubtable> = HashMap::new();
        for i in &encoding_records {
            buffer.offset = (record.offset + i.offset) as usize;
            subtables
                .entry(i.offset)
                .or_insert(buffer.get::<CmapSubtable>());
        }

        let data = HashMap::new();

        self.cmap = Some(Table_cmap {
            _version,
            num_tables,
            encoding_records,
            subtables,
            data,
        });
        self.cmap.as_ref().unwrap().format();
    }
}

#[derive(Debug)]
pub struct EncodingRecord {
    platform_id: u16,
    encoding_id: u16,
    offset: Offset32,
}

impl Read for EncodingRecord {
    fn read(buffer: &mut Buffer) -> Self {
        Self {
            platform_id: buffer.get::<u16>(),
            encoding_id: buffer.get::<u16>(),
            offset: buffer.get::<Offset32>(),
        }
    }
}

#[derive(Debug)]
pub struct CmapSubtable {
    format: u16,
    // Format 0: Byte encoding table
    length: Option<u16>,
    language: Option<u16>,
    glyph_id_array_u8: Option<Vec<u8>>,
    // Format 2: High-byte mapping through table
    sub_header_keys: Option<Vec<u16>>,
    sub_headers: Option<Vec<SubHeader>>,
    glyph_id_array: Option<Vec<u16>>,
    // Format 4: Segment mapping to delta values
    seg_count_x2: Option<u16>,
    search_range: Option<u16>,
    entry_selector: Option<u16>,
    range_shift: Option<u16>,
    end_code: Option<Vec<u16>>,
    // reserved_pad: u16,
    start_code: Option<Vec<u16>>,
    id_delta: Option<Vec<i16>>,
    id_range_offset: Option<Vec<u16>>,
    // Format 6: Trimmed table mapping
    first_code: Option<u16>,
    entry_count: Option<u16>,
    // Format 8: mixed 16-bit and 32-bit coverage
    length_u32: Option<u32>,
    language_u32: Option<u32>,
    is_32: Option<Vec<u8>>,
    num_groups: Option<u32>,
    sequential_map_groups: Option<Vec<SequentialMapGroup>>,
    // Format 10: Trimmed array
    start_char_code: Option<u32>,
    num_chars: Option<u32>,
    glyphs: Option<Vec<u16>>,
    // Format 12: Segmented coverage
    // * The same structure as Format 8
    // Format 13: Many-to-one range mappings
    constant_map_groups: Option<Vec<ConstantMapGroup>>,
    // Format 14: Unicode Variation Sequences
    num_var_selector_records: Option<u32>,
    var_selector: Option<Vec<VariationSelector>>,
}

impl Read for CmapSubtable {
    fn read(buffer: &mut Buffer) -> Self {
        let mut table = Self {
            format: buffer.get::<u16>(),
            length: None,
            language: None,
            glyph_id_array_u8: None,
            sub_header_keys: None,
            sub_headers: None,
            glyph_id_array: None,
            seg_count_x2: None,
            search_range: None,
            entry_selector: None,
            range_shift: None,
            end_code: None,
            start_code: None,
            id_delta: None,
            id_range_offset: None,
            first_code: None,
            entry_count: None,
            is_32: None,
            num_groups: None,
            sequential_map_groups: None,
            start_char_code: None,
            num_chars: None,
            glyphs: None,
            length_u32: None,
            language_u32: None,
            constant_map_groups: None,
            num_var_selector_records: None,
            var_selector: None,
        };
        match table.format {
            0 => {
                table.length = Some(buffer.get::<u16>());
                table.language = Some(buffer.get::<u16>());
                table.glyph_id_array_u8 = Some(buffer.get_vec::<u8>(256));
            }
            2 => {
                table.length = Some(buffer.get::<u16>());
                table.language = Some(buffer.get::<u16>());
                table.sub_header_keys = Some(buffer.get_vec::<u16>(256));
                let max_sub_header_key = *(table
                    .sub_header_keys
                    .as_ref()
                    .unwrap()
                    .iter()
                    .max()
                    .unwrap()) as usize;
                let mut sub_headers: Vec<SubHeader> = Vec::new();
                for _ in 0..max_sub_header_key / 8 {
                    let first_code = buffer.get::<u16>();
                    let entry_count = buffer.get::<u16>();
                    let id_delta = buffer.get::<i16>();
                    let id_range_offset = buffer.get::<u16>();
                    let offset = buffer.offset;
                    buffer.offset += id_range_offset as usize - 2;
                    let glyph_id_list = buffer
                        .get_vec::<u16>(entry_count as usize)
                        .iter()
                        .map(|x| x + id_delta as u16)
                        .collect();
                    sub_headers.push(SubHeader {
                        first_code,
                        entry_count,
                        id_delta,
                        id_range_offset,
                        glyph_id_list,
                    });
                    buffer.offset = offset;
                }
                table.sub_headers = Some(sub_headers);
            }
            4 => {
                table.length = Some(buffer.get::<u16>());
                table.language = Some(buffer.get::<u16>());
                table.seg_count_x2 = Some(buffer.get::<u16>());
                table.search_range = Some(buffer.get::<u16>());
                table.entry_selector = Some(buffer.get::<u16>());
                table.range_shift = Some(buffer.get::<u16>());
                let seg_count = table.seg_count_x2.unwrap() as usize / 2;
                table.end_code = Some(buffer.get_vec::<u16>(seg_count));
                buffer.skip::<u16>(1);
                table.start_code = Some(buffer.get_vec::<u16>(seg_count));
                table.id_delta = Some(buffer.get_vec::<i16>(seg_count));
                table.id_range_offset = Some(buffer.get_vec::<u16>(seg_count));
                // TODO: length not explicitly specified in OTF-SPEC
                table.glyph_id_array = Some(buffer.get_vec::<u16>(1));
            }
            6 => {
                table.length = Some(buffer.get::<u16>());
                table.language = Some(buffer.get::<u16>());
                table.first_code = Some(buffer.get::<u16>());
                table.entry_count = Some(buffer.get::<u16>());
                table.glyph_id_array =
                    Some(buffer.get_vec::<u16>(table.entry_count.unwrap() as usize));
            }
            8 => {
                buffer.skip::<u16>(1);
                table.length_u32 = Some(buffer.get::<u32>());
                table.language_u32 = Some(buffer.get::<u32>());
                table.is_32 = Some(buffer.get_vec::<u8>(8192));
                table.num_groups = Some(buffer.get::<u32>());
                table.sequential_map_groups =
                    Some(buffer.get_vec::<SequentialMapGroup>(table.num_groups.unwrap() as usize));
            }
            10 => {
                buffer.skip::<u16>(1);
                table.length_u32 = Some(buffer.get::<u32>());
                table.language_u32 = Some(buffer.get::<u32>());
                table.start_char_code = Some(buffer.get::<u32>());
                table.num_chars = Some(buffer.get::<u32>());
                table.glyphs = Some(buffer.get_vec::<u16>(table.num_chars.unwrap() as usize));
            }
            12 => {
                buffer.skip::<u16>(1);
                table.length_u32 = Some(buffer.get::<u32>());
                table.language_u32 = Some(buffer.get::<u32>());
                table.num_groups = Some(buffer.get::<u32>());
                table.sequential_map_groups =
                    Some(buffer.get_vec::<SequentialMapGroup>(table.num_groups.unwrap() as usize));
            }
            13 => {
                buffer.skip::<u16>(1);
                table.length_u32 = Some(buffer.get::<u32>());
                table.language_u32 = Some(buffer.get::<u32>());
                table.num_groups = Some(buffer.get::<u32>());
                table.constant_map_groups =
                    Some(buffer.get_vec::<ConstantMapGroup>(table.num_groups.unwrap() as usize));
            }
            14 => {
                table.length = Some(buffer.get::<u16>());
                table.num_var_selector_records = Some(buffer.get::<u32>());
                table.var_selector =
                    Some(buffer.get_vec::<VariationSelector>(
                        table.num_var_selector_records.unwrap() as usize,
                    ));
            }
            _ => (),
        }
        table
    }
}

#[derive(Debug)]
pub struct SubHeader {
    first_code: u16,
    entry_count: u16,
    id_delta: i16,
    id_range_offset: u16,
    // As fonttools
    glyph_id_list: Vec<u16>,
}

#[derive(Debug)]
pub struct SequentialMapGroup {
    start_char_code: u32,
    end_char_code: u32,
    start_glyph_id: u32,
}

impl Read for SequentialMapGroup {
    fn read(buffer: &mut Buffer) -> Self {
        Self {
            start_char_code: buffer.get::<u32>(),
            end_char_code: buffer.get::<u32>(),
            start_glyph_id: buffer.get::<u32>(),
        }
    }
}

#[derive(Debug)]
pub struct ConstantMapGroup {
    start_char_code: u32,
    end_char_code: u32,
    glyph_id: u32,
}

impl Read for ConstantMapGroup {
    fn read(buffer: &mut Buffer) -> Self {
        Self {
            start_char_code: buffer.get::<u32>(),
            end_char_code: buffer.get::<u32>(),
            glyph_id: buffer.get::<u32>(),
        }
    }
}

#[derive(Debug)]
pub struct VariationSelector {
    var_selector: u24,
    default_uvs_offset: Offset32,
    non_default_uvs_offset: Offset32,
}

impl Read for VariationSelector {
    fn read(buffer: &mut Buffer) -> Self {
        Self {
            var_selector: buffer.get::<u24>(),
            default_uvs_offset: buffer.get::<Offset32>(),
            non_default_uvs_offset: buffer.get::<Offset32>(),
        }
    }
}
