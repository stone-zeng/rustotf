use crate::table::{head::Table_head, hhea::Table_hhea, maxp::Table_maxp};
use crate::util::Buffer;
use std::collections::HashMap;

#[allow(non_snake_case)]
#[derive(Debug)]
pub struct Font {
    subtype: Subtype,

    // Required Tables
    // cmap: Option<Table_cmap>, // Character to glyph mapping
    head: Option<Table_head>, // Font header
    hhea: Option<Table_hhea>, // Horizontal header
    // hmtx: Option<Table_hmtx>, // Horizontal metrics
    maxp: Option<Table_maxp>, // Maximum profile
    // name: Option<Table_name>, // Naming table
    // OS_2: Option<Table_OS_2>, // OS/2 and Windows specific metrics
    // post: Option<Table_post>, // PostScript information

    // // Tables Related to TrueType Outlines
    // cvt_: Option<Table_cvt_>, // Control Value Table (optional table)
    // fpgm: Option<Table_fpgm>, // Font program (optional table)
    // glyf: Option<Table_glyf>, // Glyph data
    // loca: Option<Table_loca>, // Index to location
    // prep: Option<Table_prep>, // CVT Program (optional table)
    // gasp: Option<Table_gasp>, // Grid-fitting/Scan-conversion (optional table)

    // // Tables Related to CFF Outlines
    // CFF_: Option<Table_CFF_>, // Compact Font Format 1.0
    // CFF2: Option<Table_CFF2>, // Compact Font Format 2.0
    // VORG: Option<Table_VORG>, // Vertical Origin (optional table)

    // // Tables Related to Bitmap Glyphs
    // EBDT: Option<Table_EBDT>, // Embedded bitmap data
    // EBLC: Option<Table_EBLC>, // Embedded bitmap location data
    // EBSC: Option<Table_EBSC>, // Embedded bitmap scaling data
    // CBDT: Option<Table_CBDT>, // Color bitmap data
    // CBLC: Option<Table_CBLC>, // Color bitmap location data
    // sbix: Option<Table_sbix>, // Standard bitmap graphics

    // // Advanced Typographic Tables
    // BASE: Option<Table_BASE>, // Baseline data
    // GDEF: Option<Table_GDEF>, // Glyph definition data
    // GPOS: Option<Table_GPOS>, // Glyph positioning data
    // GSUB: Option<Table_GSUB>, // Glyph substitution data
    // JSTF: Option<Table_JSTF>, // Justification data
    // MATH: Option<Table_MATH>, // Math layout data

    // // Tables used for OpenType Font Variations
    // avar: Option<Table_avar>, // Axis variations
    // cvar: Option<Table_cvar>, // CVT variations (TrueType outlines only)
    // fvar: Option<Table_fvar>, // Font variations
    // gvar: Option<Table_gvar>, // Glyph variations (TrueType outlines only)
    // HVAR: Option<Table_HVAR>, // Horizontal metrics variations
    // MVAR: Option<Table_MVAR>, // Metrics variations
    // STAT: Option<Table_STAT>, // Style attributes (required for variable fonts, optional for non-variable fonts)
    // VVAR: Option<Table_VVAR>, // Vertical metrics variations

    // // Tables Related to Color Fonts
    // COLR: Option<Table_COLR>, // Color table
    // CPAL: Option<Table_CPAL>, // Color palette table
    // SVG_: Option<Table_SVG_>, // The SVG (Scalable Vector Graphics) table

    // // Other OpenType Tables
    // DSIG: Option<Table_DSIG>, // Digital signature
    // hdmx: Option<Table_hdmx>, // Horizontal device metrics
    // kern: Option<Table_kern>, // Kerning
    // LTSH: Option<Table_LTSH>, // Linear threshold data
    // MERG: Option<Table_MERG>, // Merge
    // meta: Option<Table_meta>, // Metadata
    // PCLT: Option<Table_PCLT>, // PCL 5 data
    // VDMX: Option<Table_VDMX>, // Vertical device metrics
    // vhea: Option<Table_vhea>, // Vertical Metrics header
    // vmtx: Option<Table_vmtx>, // Vertical Metrics
}

impl Font {
    fn new(subtype: Subtype) -> Self {
        Self {
            subtype,
            head: None,
            hhea: None,
            maxp: None,
        }
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
    pub offset: u32,
    pub length: u32,
}

pub trait FontTable {
    fn parse(buffer: &mut Buffer, record: &TableRecord) -> Self;
}

// struct Table_cmap {}

// struct Table_hhea {}
// struct Table_hmtx {}
// struct Table_maxp {}
// struct Table_name {}
// struct Table_OS_2 {}
// struct Table_post {}
// struct Table_cvt_ {}
// struct Table_fpgm {}
// struct Table_glyf {}
// struct Table_loca {}
// struct Table_prep {}
// struct Table_gasp {}
// struct Table_CFF_ {}
// struct Table_CFF2 {}
// struct Table_VORG {}
// struct Table_EBDT {}
// struct Table_EBLC {}
// struct Table_EBSC {}
// struct Table_CBDT {}
// struct Table_CBLC {}
// struct Table_sbix {}
// struct Table_BASE {}
// struct Table_GDEF {}
// struct Table_GPOS {}
// struct Table_GSUB {}
// struct Table_JSTF {}
// struct Table_MATH {}
// struct Table_avar {}
// struct Table_cvar {}
// struct Table_fvar {}
// struct Table_gvar {}
// struct Table_HVAR {}
// struct Table_MVAR {}
// struct Table_STAT {}
// struct Table_VVAR {}
// struct Table_COLR {}
// struct Table_CPAL {}
// struct Table_SVG_ {}
// struct Table_DSIG {}
// struct Table_hdmx {}
// struct Table_kern {}
// struct Table_LTSH {}
// struct Table_MERG {}
// struct Table_meta {}
// struct Table_PCLT {}
// struct Table_VDMX {}
// struct Table_vhea {}
// struct Table_vmtx {}

pub fn read_otf(buffer: &mut Buffer, signature: u32) {
    // Offset Table
    let num_tables = buffer.read_u16();
    let search_range = buffer.read_u16();
    let entry_selector = buffer.read_u16();
    let range_shift = buffer.read_u16();
    println!(
        "\tnumTables: {}\n\tsearchRange: {}\n\tentrySelector: {}\n\trangeShift: {}",
        num_tables, search_range, entry_selector, range_shift
    );

    let mut font = Font::new(match signature {
        // 'OTTO'
        0x4F54_544F => Subtype::CFF,
        _ => Subtype::TTF,
    });

    // Table Record entries
    let mut records = HashMap::new();
    for _ in 0..num_tables {
        records.insert(
            buffer.read_tag(),
            TableRecord {
                check_sum: buffer.read_u32(),
                offset: buffer.read_u32(),
                length: buffer.read_u32(),
            },
        );
    }

    font.head = Some(Table_head::parse(buffer, records.get("head").unwrap()));
    font.hhea = Some(Table_hhea::parse(buffer, records.get("hhea").unwrap()));
    // font.hmtx = Some(Table_hmtx::parse(buffer, records.get("hmtx").unwrap()));
    font.maxp = Some(Table_maxp::parse(buffer, records.get("maxp").unwrap()));

    // for r in records {
    //     match r.table_tag.as_ref() {
    //         "head" => font.head = Some(Table_head::parse(buffer, &r)),
    //         "hhea" => font.hhea = Some(Table_hhea::parse(buffer, &r)),
    //         _ => (),
    //     }
    // }
    // println!(
    //     "\ttableTag: {}\n\t\tcheckSum: [{}, {}]\n\t\toffset: {}\n\t\tlength: {}",
    //     table_tag,
    //     check_sum,
    //     buffer.calc_check_sum(offset, length),
    //     offset,
    //     length
    // );

    println!("{:?}", font);
}

pub fn read_ttc(buffer: &mut Buffer) {
    // TTC Header
    let major_version = buffer.read_u16();
    let minor_version = buffer.read_u16();
    let num_fonts = buffer.read_u32();
    let mut offset_table: Vec<u32> = Vec::new();
    for _ in 0..num_fonts {
        offset_table.push(buffer.read_u32());
    }

    println!(
        "\tmajorVersion: {}\n\tminorVersion: {}\n\tnumFonts: {}\n\toffsetTable: {:?}",
        major_version, minor_version, num_fonts, offset_table
    );

    if major_version == 2 {
        let dsig_tag = buffer.read_u32();
        let dsig_length = buffer.read_u32();
        let dsig_offset = buffer.read_u32();
        println!(
            "\tdsigTag: {}\n\tdsigLength: {}\n\tdsigOffset: {}",
            dsig_tag, dsig_length, dsig_offset
        );
    }
}

pub fn read_woff(buffer: &mut Buffer) {
    let flavor = buffer.read_u32();
    let length = buffer.read_u32();
    let num_tables = buffer.read_u16();
    let reserved = buffer.read_u16();
    let total_sfnt_size = buffer.read_u32();
    let major_version = buffer.read_u16();
    let minor_version = buffer.read_u16();
    let meta_offset = buffer.read_u32();
    let meta_length = buffer.read_u32();
    let meta_orig_length = buffer.read_u32();
    let priv_offset = buffer.read_u32();
    let priv_length = buffer.read_u32();
    println!(
        "\tflavor: {:08X}\n\tlength: {}\n\tnumTables: {}\n\treserved: {}\n\ttotalSfntSize: {}\n\tmajorVersion: {}\n\tminorVersion: {}\n\tmetaOffset: {}\n\tmetaLength: {}\n\tmetaOrigLength: {}\n\tprivOffset: {}\n\tprivLength: {}",
        flavor, length, num_tables, reserved, total_sfnt_size, major_version, minor_version, meta_offset, meta_length, meta_orig_length, priv_offset, priv_length);
}

pub fn read_woff2(buffer: &mut Buffer) {
    let flavor = buffer.read_u32();
    let length = buffer.read_u32();
    let num_tables = buffer.read_u16();
    let reserved = buffer.read_u16();
    let total_sfnt_size = buffer.read_u32();
    let total_compressed_size = buffer.read_u32();
    let major_version = buffer.read_u16();
    let minor_version = buffer.read_u16();
    let meta_offset = buffer.read_u32();
    let meta_length = buffer.read_u32();
    let meta_orig_length = buffer.read_u32();
    let priv_offset = buffer.read_u32();
    let priv_length = buffer.read_u32();
    println!(
        "\tflavor: {:08X}\n\tlength: {}\n\tnumTables: {}\n\treserved: {}\n\ttotalSfntSize: {}\n\ttotalCompressedSize: {}\n\tmajorVersion: {}\n\tminorVersion: {}\n\tmetaOffset: {}\n\tmetaLength: {}\n\tmetaOrigLength: {}\n\tprivOffset: {}\n\tprivLength: {}",
        flavor, length, num_tables, reserved, total_sfnt_size, total_compressed_size, major_version, minor_version, meta_offset, meta_length, meta_orig_length, priv_offset, priv_length);
}
