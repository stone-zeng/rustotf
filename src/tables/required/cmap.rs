use crate::font::Font;
use crate::types::u24;
use crate::util::{Buffer, ReadBuffer};
use read_buffer_derive::ReadBuffer;
use std::collections::HashMap;

/// ## `cmap` &mdash; Character to Glyph Index Mapping Table
///
/// Specification: <https://docs.microsoft.com/en-us/typography/opentype/spec/cmap>.
///
/// This table defines the mapping of character codes to the glyph index values
/// used in the font. It may contain more than one subtable, in order to support
/// more than one character encoding scheme.
///
/// TODO: map is planned to be a `HashMap` of `cid` => `gid`. Not finished yet.

#[allow(non_camel_case_types)]
#[derive(Debug)]
pub struct Table_cmap {
    version: u16,
    num_tables: u16,
    encodings: Vec<Encoding>,
    subtables: HashMap<(u16, u16), CmapSubtable>,
    pub maps: HashMap<Encoding, Map>,
}

impl Font {
    pub fn parse_cmap(&mut self, buffer: &mut Buffer) {
        let start = buffer.offset();
        let version = buffer.get();
        let num_tables = buffer.get();
        let encodings: Vec<Encoding> = buffer.get_vec(num_tables);
        let subtables = encodings
            .iter()
            .map(|i| {
                buffer.set_offset_from(start, i.offset);
                ((i.platform_id, i.encoding_id), buffer.get())
            })
            .collect();

        // TODO: parse maps
        let maps: HashMap<Encoding, Map> = HashMap::new();

        self.cmap = Some(Table_cmap {
            version,
            num_tables,
            encodings,
            subtables,
            maps,
        });
    }
}

#[derive(Clone, Debug, Eq, Hash, PartialEq, ReadBuffer)]
pub struct Encoding {
    pub platform_id: u16,
    pub encoding_id: u16,
    offset: u32,
}

#[derive(Debug, Default)]
struct CmapSubtable {
    format: u16,
    format_0_data: Option<CmapFormat0>,
    format_2_data: Option<CmapFormat2>,
    format_4_data: Option<CmapFormat4>,
    format_6_data: Option<CmapFormat6>,
    format_8_data: Option<CmapFormat8>,
    format_10_data: Option<CmapFormat10>,
    format_12_data: Option<CmapFormat12>,
    format_13_data: Option<CmapFormat13>,
    format_14_data: Option<CmapFormat14>,
}

impl ReadBuffer for CmapSubtable {
    fn read(buffer: &mut Buffer) -> Self {
        let mut subtable = CmapSubtable {
            format: buffer.get(),
            ..Default::default()
        };
        match subtable.format {
            0 => subtable.format_0_data = Some(buffer.get()),
            2 => subtable.format_2_data = Some(buffer.get()),
            4 => subtable.format_4_data = Some(buffer.get()),
            6 => subtable.format_6_data = Some(buffer.get()),
            8 => subtable.format_8_data = Some(buffer.get()),
            10 => subtable.format_10_data = Some(buffer.get()),
            12 => subtable.format_12_data = Some(buffer.get()),
            13 => subtable.format_13_data = Some(buffer.get()),
            14 => subtable.format_14_data = Some(buffer.get()),
            _ => unreachable!(),
        }
        subtable
    }
}

#[derive(Debug)]
struct CmapFormat0 {
    length: u16,
    language: u16,
    gid_array: Vec<u8>, // glyphIdArray[256]
    map: Map,
}

impl ReadBuffer for CmapFormat0 {
    fn read(buffer: &mut Buffer) -> Self {
        let length = buffer.get();
        let language = buffer.get();
        let gid_array = buffer.get_vec(256);
        let map = (0..256)
            .zip(gid_array.iter())
            .map(|(cid, &gid)| (cid, u32::from(gid)))
            .collect();
        Self {
            length,
            language,
            gid_array,
            map,
        }
    }
}

#[derive(Debug)]
struct CmapFormat2 {
    length: u16,
    language: u16,
    sub_header_keys: Vec<u16>, // subHeaderKeys[256]
    sub_headers: Vec<SubHeader>,
    // glyphIndexArray[] is in `SubHeader`
    map: Map,
}

impl ReadBuffer for CmapFormat2 {
    fn read(buffer: &mut Buffer) -> Self {
        let length = buffer.get();
        let language = buffer.get();
        let sub_header_keys = buffer.get_vec(256);
        let max_sub_header_key = sub_header_keys.iter().max().unwrap();
        let mut sub_headers: Vec<SubHeader> = Vec::new();
        for _ in 0..max_sub_header_key / 8 {
            let first_code = buffer.get();
            let entry_count = buffer.get();
            let id_delta = buffer.get();
            let id_range_offset = buffer.get();
            let offset = buffer.offset();
            buffer.advance_offset(id_range_offset - 2);
            let gid_array = buffer
                .get_vec(entry_count)
                .iter()
                .map(|x| x + id_delta as u16)
                .collect();
            sub_headers.push(SubHeader {
                first_code,
                entry_count,
                id_delta,
                id_range_offset,
                gid_array,
            });
            buffer.set_offset(offset);
        }
        Self {
            length,
            language,
            sub_header_keys,
            sub_headers,
            map: Map::new(),
        }
    }
}

#[derive(Debug)]
struct CmapFormat4 {
    length: u16,
    language: u16,
    seg_count_x2: u16,
    search_range: u16,
    entry_selector: u16,
    range_shift: u16,
    end_char_code: Vec<u16>,   // endCode[segCount]
    start_char_code: Vec<u16>, // startCode[segCount]
    id_delta: Vec<i16>,
    id_range_offset: Vec<u16>,
    map: Map,
}

impl ReadBuffer for CmapFormat4 {
    fn read(buffer: &mut Buffer) -> Self {
        let length = buffer.get();
        let language = buffer.get();
        let seg_count_x2 = buffer.get();
        let search_range = buffer.get();
        let entry_selector = buffer.get();
        let range_shift = buffer.get();
        let seg_count = seg_count_x2 as usize / 2;
        let end_char_code = buffer.get_vec(seg_count);
        let start_char_code = {
            buffer.skip::<u16>(1);
            buffer.get_vec(seg_count)
        };
        let id_delta = buffer.get_vec(seg_count);
        let id_range_offset_begin_offset = buffer.offset();
        let id_range_offset = buffer.get_vec(seg_count);

        let mut gid_seg_array = Vec::new();
        let mut map = HashMap::new();

        for i in 0..seg_count {
            let start = start_char_code[i] as u32;
            let end = end_char_code[i] as u32;
            let filtered_char_range = (start..=end).filter(|&c| c < 0xFFFF);
            let gid_seg: Vec<u32> = if id_range_offset[i] != 0 {
                filtered_char_range
                    .map(|c| {
                        // TODO: otfcc is different from the C code in MS reference page.
                        buffer.set_offset(
                            // Address of `id_range_offset[i]`...
                            id_range_offset_begin_offset + i * 2
                            // ... plus some offset
                            + (id_range_offset[i] as u32 + (c - start) * 2) as usize,
                        );
                        ((buffer.get::<u16>() as i32 + id_delta[i] as i32) % 0xFFFF) as u32
                    })
                    .collect()
            } else {
                filtered_char_range
                    .map(|c| ((c as i32 + id_delta[i] as i32) % 0xFFFF) as u32)
                    .collect()
            };
            gid_seg_array.push(gid_seg.to_vec());
            (start..=end).zip(gid_seg.iter()).for_each(|(cid, &gid)| {
                map.insert(cid, gid);
            });
        }

        Self {
            length,
            language,
            seg_count_x2,
            search_range,
            entry_selector,
            range_shift,
            end_char_code,
            start_char_code,
            id_delta,
            id_range_offset,
            map,
        }
    }
}

#[derive(Debug)]
struct CmapFormat6 {
    length: u16,
    language: u16,
    start_char_code: u16, // firstCode
    entry_count: u16,
    gid_array: Vec<u16>,
    map: Map,
}

impl ReadBuffer for CmapFormat6 {
    fn read(buffer: &mut Buffer) -> Self {
        let length = buffer.get();
        let language = buffer.get();
        let start_char_code = buffer.get();
        let entry_count = buffer.get();
        let gid_array = buffer.get_vec(entry_count);
        Self {
            length,
            language,
            start_char_code,
            entry_count,
            gid_array,
            map: Map::new(),
        }
    }
}

#[derive(Debug)]
struct CmapFormat8 {
    length: u32,
    language: u32,
    is_32: Vec<u8>,
    num_groups: u32,
    groups: Vec<SequentialMapGroup>,
    map: Map,
}

impl ReadBuffer for CmapFormat8 {
    fn read(buffer: &mut Buffer) -> Self {
        buffer.skip::<u16>(1);
        let length = buffer.get();
        let language = buffer.get();
        let is_32 = buffer.get_vec(8192);
        let num_groups = buffer.get();
        let groups = buffer.get_vec(num_groups);
        Self {
            length,
            language,
            is_32,
            num_groups,
            groups,
            map: Map::new(),
        }
    }
}

#[derive(Debug)]
struct CmapFormat10 {
    length: u32,
    language: u32,
    start_char_code: u32,
    entry_count: u32,    // numChars
    gid_array: Vec<u16>, // glyphs[]
    map: Map,
}

impl ReadBuffer for CmapFormat10 {
    fn read(buffer: &mut Buffer) -> Self {
        buffer.skip::<u16>(1);
        let length = buffer.get();
        let language = buffer.get();
        let start_char_code = buffer.get();
        let entry_count = buffer.get();
        let gid_array = buffer.get_vec(entry_count);
        Self {
            length,
            language,
            start_char_code,
            entry_count,
            gid_array,
            map: Map::new(),
        }
    }
}

#[derive(Debug)]
struct CmapFormat12 {
    length: u32,
    language: u32,
    num_groups: u32,
    groups: Vec<SequentialMapGroup>,
    map: Map,
}

impl ReadBuffer for CmapFormat12 {
    fn read(buffer: &mut Buffer) -> Self {
        buffer.skip::<u16>(1);
        let length = buffer.get();
        let language = buffer.get();
        let num_groups = buffer.get();
        let groups = buffer.get_vec(num_groups);
        Self {
            length,
            language,
            num_groups,
            groups,
            map: Map::new(),
        }
    }
}

#[derive(Debug)]
struct CmapFormat13 {
    length: u32,
    language: u32,
    num_groups: u32,
    groups: Vec<ConstantMapGroup>,
    map: Map,
}

impl ReadBuffer for CmapFormat13 {
    fn read(buffer: &mut Buffer) -> Self {
        buffer.skip::<u16>(1);
        let length = buffer.get();
        let language = buffer.get();
        let num_groups = buffer.get();
        let groups = buffer.get_vec(num_groups);
        Self {
            length,
            language,
            num_groups,
            groups,
            map: Map::new(),
        }
    }
}

#[derive(Debug)]
struct CmapFormat14 {
    length: u32,
    num_var_selectors: u32,
    var_selectors: Vec<VariationSelector>,
    map: Map,
}

impl ReadBuffer for CmapFormat14 {
    fn read(buffer: &mut Buffer) -> Self {
        let length = buffer.get();
        let num_var_selectors = buffer.get();
        let var_selectors = buffer.get_vec(num_var_selectors);
        Self {
            length,
            num_var_selectors,
            var_selectors,
            map: Map::new(),
        }
    }
}

#[derive(Debug)]
struct SubHeader {
    first_code: u16,
    entry_count: u16,
    id_delta: i16,
    id_range_offset: u16,
    gid_array: Vec<u16>, // As fonttools
}

#[derive(Debug, ReadBuffer)]
struct SequentialMapGroup {
    start_char_code: u32,
    end_char_code: u32,
    start_glyph_id: u32,
}

#[derive(Debug, ReadBuffer)]
struct ConstantMapGroup {
    start_char_code: u32,
    end_char_code: u32,
    glyph_id: u32,
}

#[derive(Debug, ReadBuffer)]
struct VariationSelector {
    var_selector: u24,
    default_uvs_offset: u32,
    non_default_uvs_offset: u32,
}

type Map = HashMap<u32, u32>;
