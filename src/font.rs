use crate::table::{
    cmap::Table_cmap, head::Table_head, hhea::Table_hhea, hmtx::Table_hmtx, maxp::Table_maxp,
    name::Table_name, os_2::Table_OS_2, post::Table_post,
};
use crate::util::{Buffer, Offset32, Read, Tag};
use std::collections::HashMap;

#[allow(non_snake_case)]
#[derive(Debug)]
pub struct Font {
    subtype: Subtype,

    // Required Tables
    pub head: Option<Table_head>, // Font header
    pub hhea: Option<Table_hhea>, // Horizontal header
    pub maxp: Option<Table_maxp>, // Maximum profile
    pub hmtx: Option<Table_hmtx>, // Horizontal metrics
    pub cmap: Option<Table_cmap>, // Character to glyph mapping
    pub name: Option<Table_name>, // Naming table
    pub OS_2: Option<Table_OS_2>, // OS/2 and Windows specific metrics
    pub post: Option<Table_post>, // PostScript information

    // // Tables Related to TrueType Outlines
    // pub cvt_: Option<Table_cvt_>, // Control Value Table (optional table)
    // pub fpgm: Option<Table_fpgm>, // Font program (optional table)
    // pub glyf: Option<Table_glyf>, // Glyph data
    // pub loca: Option<Table_loca>, // Index to location
    // pub prep: Option<Table_prep>, // CVT Program (optional table)
    // pub gasp: Option<Table_gasp>, // Grid-fitting/Scan-conversion (optional table)

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
            hmtx: None,
            cmap: None,
            name: None,
            OS_2: None,
            post: None,
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
    pub offset: Offset32,
    pub length: u32,
}

impl Read for TableRecord {
    fn read(buffer: &mut Buffer) -> Self {
        Self {
            check_sum: buffer.get::<u32>(),
            offset: buffer.get::<Offset32>(),
            length: buffer.get::<u32>(),
        }
    }
}

pub fn read_otf(buffer: &mut Buffer, signature: u32) {
    // Offset Table
    let num_tables = buffer.get::<u16>();
    let search_range = buffer.get::<u16>();
    let entry_selector = buffer.get::<u16>();
    let range_shift = buffer.get::<u16>();
    // println!(
    //     "\tnumTables: {}\n\tsearchRange: {}\n\tentrySelector: {}\n\trangeShift: {}",
    //     num_tables, search_range, entry_selector, range_shift
    // );

    let mut font = Font::new(match signature {
        // 'OTTO'
        0x4F54_544F => Subtype::CFF,
        _ => Subtype::TTF,
    });

    // Table Record entries
    let mut records = HashMap::new();
    for _ in 0..num_tables {
        records.insert(buffer.get::<Tag>().to_string(), buffer.get::<TableRecord>());
    }

    // println!("{:#?}", records);

    font.parse_head(buffer, records.get("head").unwrap());
    font.parse_hhea(buffer, records.get("hhea").unwrap());
    font.parse_maxp(buffer, records.get("maxp").unwrap());
    font.parse_hmtx(buffer, records.get("hmtx").unwrap());
    font.parse_cmap(buffer, records.get("cmap").unwrap());
    font.parse_name(buffer, records.get("name").unwrap());
    font.parse_OS_2(buffer, records.get("OS/2").unwrap());
    font.parse_post(buffer, records.get("post").unwrap());

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
}

pub fn read_woff(buffer: &mut Buffer) {
    let flavor = buffer.get::<u32>();
    let length = buffer.get::<u32>();
    let num_tables = buffer.get::<u16>();
    let reserved = buffer.get::<u16>();
    let total_sfnt_size = buffer.get::<u32>();
    let major_version = buffer.get::<u16>();
    let minor_version = buffer.get::<u16>();
    let meta_offset = buffer.get::<u32>();
    let meta_length = buffer.get::<u32>();
    let meta_orig_length = buffer.get::<u32>();
    let priv_offset = buffer.get::<u32>();
    let priv_length = buffer.get::<u32>();
    println!(
        "\tflavor: {:08X}\n\tlength: {}\n\tnumTables: {}\n\treserved: {}\n\ttotalSfntSize: {}\n\tmajorVersion: {}\n\tminorVersion: {}\n\tmetaOffset: {}\n\tmetaLength: {}\n\tmetaOrigLength: {}\n\tprivOffset: {}\n\tprivLength: {}",
        flavor, length, num_tables, reserved, total_sfnt_size, major_version, minor_version, meta_offset, meta_length, meta_orig_length, priv_offset, priv_length);
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
}
