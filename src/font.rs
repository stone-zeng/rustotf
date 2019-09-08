use crate::table::{
    cmap::Table_cmap, head::Table_head, hhea::Table_hhea, hmtx::Table_hmtx, maxp::Table_maxp,
    name::Table_name, os_2::Table_OS_2, post::Table_post,
};
use crate::util::{Buffer, Offset32, ReadBuffer, Tag};

use std::collections::HashMap;
use std::io::Read;

use flate2::read::ZlibDecoder;

#[allow(non_snake_case)]
#[derive(Debug)]
pub struct Font {
    subtype: Subtype,

    table_records: HashMap<String, TableRecord>,
    woff_table_records: HashMap<String, WoffTableRecord>,

    // Required Tables
    pub head: Option<Table_head>, // Font header
    pub hhea: Option<Table_hhea>, // Horizontal header
    pub maxp: Option<Table_maxp>, // Maximum profile
    pub hmtx: Option<Table_hmtx>, // Horizontal metrics
    pub cmap: Option<Table_cmap>, // Character to glyph mapping
    pub name: Option<Table_name>, // Naming table
    pub OS_2: Option<Table_OS_2>, // OS/2 and Windows specific metrics
    pub post: Option<Table_post>, // PostScript information
}

impl Font {
    fn new(signature: u32) -> Self {
        Self {
            subtype: match signature {
                0x4F54_544F => Subtype::CFF,
                0x0001_0000 => Subtype::TTF,
                _ => Subtype::TTF,
            },
            table_records: HashMap::new(),
            woff_table_records: HashMap::new(),
            head: None,
            hhea: None,
            maxp: None,
            hmtx: None,
            cmap: None,
            name: None,
            OS_2: None,
            post: None,
        }
    }

    fn _parse(&mut self, table_tag: &str, buffer: &mut Buffer) {
        match table_tag {
            "head" => self.parse_head(buffer),
            "hhea" => self.parse_hhea(buffer),
            "maxp" => self.parse_maxp(buffer),
            "hmtx" => self.parse_hmtx(buffer),
            "cmap" => self.parse_cmap(buffer),
            "name" => self.parse_name(buffer),
            "OS/2" => self.parse_OS_2(buffer),
            "post" => self.parse_post(buffer),
            _ => (),
        };
    }

    fn parse(&mut self, table_tag: &str, buffer: &mut Buffer) {
        buffer.offset = self.get_record(table_tag).offset as usize;
        self._parse(table_tag, buffer);
    }

    fn parse_woff(&mut self, table_tag: &str, buffer: &mut Buffer) {
        let start = self.woff_table_records.get(table_tag).unwrap().offset as usize;
        let end = start + self.woff_table_records.get(table_tag).unwrap().comp_length as usize;
        let mut new_buffer = decompress_woff(buffer, start, end);

        println!("----- {}", table_tag);

        self._parse(table_tag, &mut new_buffer);
    }

    pub fn get_record(&self, table_tag: &str) -> &TableRecord {
        self.table_records.get(table_tag).unwrap()
    }
}

#[derive(Debug)]
enum Subtype {
    TTF,
    CFF,
}

#[derive(Debug)]
pub struct TableRecord {
    pub check_sum: u32,
    pub offset: Offset32,
    pub length: u32,
}

impl ReadBuffer for TableRecord {
    fn read(buffer: &mut Buffer) -> Self {
        Self {
            check_sum: buffer.get::<u32>(),
            offset: buffer.get::<Offset32>(),
            length: buffer.get::<u32>(),
        }
    }
}

#[derive(Debug)]
pub struct WoffTableRecord {
    offset: u32,
    comp_length: u32,
    orig_length: u32,
    orig_check_sum: u32,
}

impl ReadBuffer for WoffTableRecord {
    fn read(buffer: &mut Buffer) -> Self {
        Self {
            offset: buffer.get::<u32>(),
            comp_length: buffer.get::<u32>(),
            orig_length: buffer.get::<u32>(),
            orig_check_sum: buffer.get::<u32>(),
        }
    }
}

pub fn read_otf(buffer: &mut Buffer, signature: u32) {
    // Offset Table
    let num_tables = buffer.get::<u16>();
    // TODO: not used now
    let _search_range = buffer.get::<u16>();
    let _entry_selector = buffer.get::<u16>();
    let _range_shift = buffer.get::<u16>();
    // println!(
    //     "\tnumTables: {}\n\tsearchRange: {}\n\tentrySelector: {}\n\trangeShift: {}",
    //     num_tables, search_range, entry_selector, range_shift
    // );

    let mut font = Font::new(signature);
    for _ in 0..num_tables {
        font.table_records
            .insert(buffer.get::<Tag>().to_string(), buffer.get::<TableRecord>());
    }

    font.parse("head", buffer);
    font.parse("hhea", buffer);
    font.parse("maxp", buffer);
    font.parse("hmtx", buffer);
    font.parse("cmap", buffer);
    font.parse("name", buffer);
    font.parse("OS/2", buffer);
    font.parse("post", buffer);

    println!("\t{:#?}", font);
}

pub fn read_ttc(buffer: &mut Buffer) {
    // TTC Header
    let major_version = buffer.get::<u16>();
    let minor_version = buffer.get::<u16>();
    let num_fonts = buffer.get::<u32>();
    let offset_table = buffer.get_vec::<Offset32>(num_fonts as usize);

    println!(
        "\tmajorVersion: {}\n\tminorVersion: {}\n\tnumFonts: {}\n\toffsetTable: {:?}",
        major_version, minor_version, num_fonts, offset_table
    );

    if major_version == 2 {
        let dsig_tag = buffer.get::<u32>();
        let dsig_length = buffer.get::<u32>();
        let dsig_offset = buffer.get::<u32>();
        println!(
            "\tdsigTag: {}\n\tdsigLength: {}\n\tdsigOffset: {}",
            dsig_tag, dsig_length, dsig_offset
        );
    }

    for offset in offset_table {
        buffer.offset = offset as usize;
        let signature = buffer.get::<u32>();
        read_otf(buffer, signature);
    }
}

pub fn read_woff(buffer: &mut Buffer) {
    let flavor = buffer.get::<u32>();
    // TODO: not used now
    let _length = buffer.get::<u32>();
    let num_tables = buffer.get::<u16>();
    let _reserved = buffer.get::<u16>();
    let _total_sfnt_size = buffer.get::<u32>();
    let _major_version = buffer.get::<u16>();
    let _minor_version = buffer.get::<u16>();
    let _meta_offset = buffer.get::<u32>();
    let _meta_length = buffer.get::<u32>();
    let _meta_orig_length = buffer.get::<u32>();
    let _priv_offset = buffer.get::<u32>();
    let _priv_length = buffer.get::<u32>();
    // println!(
    //     "\tflavor: {:08X}\n\tlength: {}\n\tnumTables: {}\n\treserved: {}\n\ttotalSfntSize: {}\n\tmajorVersion: {}\n\tminorVersion: {}\n\tmetaOffset: {}\n\tmetaLength: {}\n\tmetaOrigLength: {}\n\tprivOffset: {}\n\tprivLength: {}",
    //     flavor, length, num_tables, reserved, total_sfnt_size, major_version, minor_version, meta_offset, meta_length, meta_orig_length, priv_offset, priv_length);

    let mut font = Font::new(flavor);
    for _ in 0..num_tables {
        font.woff_table_records.insert(
            buffer.get::<Tag>().to_string(),
            buffer.get::<WoffTableRecord>(),
        );
    }

    font.parse_woff("head", buffer);
    font.parse_woff("hhea", buffer);
    font.parse_woff("maxp", buffer);
    // FIXME: index out of range for slice
    // font.parse_woff("hmtx", buffer);
    font.parse_woff("cmap", buffer);
    font.parse_woff("name", buffer);
    // font.parse_woff("OS/2", buffer);
    font.parse_woff("post", buffer);

    println!("\t{:#?}", font);
}

fn decompress_woff(buffer: &mut Buffer, start: usize, end: usize) -> Buffer {
    buffer.offset = 0;
    let comp_buffer = buffer.slice(start, end);
    let mut orig_buffer: Vec<u8> = Vec::new();

    if ZlibDecoder::new(comp_buffer)
        .read_to_end(&mut orig_buffer)
        .is_ok()
    {
        Buffer::new(orig_buffer)
    } else {
        Buffer::new(comp_buffer.to_vec())
    }
}

pub fn read_woff2(buffer: &mut Buffer) {
    let flavor = buffer.get::<u32>();
    let length = buffer.get::<u32>();
    let num_tables = buffer.get::<u16>();
    let reserved = buffer.get::<u16>();
    let total_sfnt_size = buffer.get::<u32>();
    let total_compressed_size = buffer.get::<u32>();
    let major_version = buffer.get::<u16>();
    let minor_version = buffer.get::<u16>();
    let meta_offset = buffer.get::<u32>();
    let meta_length = buffer.get::<u32>();
    let meta_orig_length = buffer.get::<u32>();
    let priv_offset = buffer.get::<u32>();
    let priv_length = buffer.get::<u32>();
    println!(
        "\tflavor: {:08X}\n\tlength: {}\n\tnumTables: {}\n\treserved: {}\n\ttotalSfntSize: {}\n\ttotalCompressedSize: {}\n\tmajorVersion: {}\n\tminorVersion: {}\n\tmetaOffset: {}\n\tmetaLength: {}\n\tmetaOrigLength: {}\n\tprivOffset: {}\n\tprivLength: {}",
        flavor, length, num_tables, reserved, total_sfnt_size, total_compressed_size, major_version, minor_version, meta_offset, meta_length, meta_orig_length, priv_offset, priv_length);

    let font = Font::new(flavor);
    println!("\t{:#?}", font);
}
