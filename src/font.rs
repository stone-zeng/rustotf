use crate::tables::*;
use crate::util::{Buffer, Tag};

use std::collections::HashMap;
use std::error::Error;
use std::fs;

pub fn read_font(font_file_path: &str) -> Result<(), Box<dyn Error>> {
    // TODO: check extension.
    let mut font_container = FontContainer::new(fs::read(font_file_path)?);
    font_container.init();
    font_container.parse();
    // TODO: for debug
    for i in &font_container.fonts {
        // println!("{:#?}", i.table_records);
        println!("\"MATH\": {:#?}", i.MATH);
    }
    Ok(())
}

#[derive(Debug)]
pub struct FontContainer {
    buffer: Buffer,
    pub fonts: Vec<Font>,
}

impl FontContainer {
    pub fn new(raw_buffer: Vec<u8>) -> Self {
        Self {
            buffer: Buffer::new(raw_buffer),
            fonts: Vec::new(),
        }
    }

    pub fn init(&mut self) {
        let signature = self.buffer.get();
        self.buffer.offset = 0;
        match signature {
            SIGNATURE_OTF | SIGNATURE_TTF | SIGNATURE_TTF_TRUE | SIGNATURE_TTF_TYP1 => {
                self.otf_init()
            }
            SIGNATURE_TTC => self.ttc_init(),
            SIGNATURE_WOFF => self.woff_init(),
            SIGNATURE_WOFF2 => self.woff2_init(),
            _ => unreachable!(),
        }
    }

    fn otf_init(&mut self) {
        self.fonts.push(Font::load_sfnt(&mut self.buffer));
    }

    #[allow(unused_variables)]
    fn ttc_init(&mut self) {
        let ttc_tag: u32 = self.buffer.get();
        let major_version: u16 = self.buffer.get();
        let minor_version: u16 = self.buffer.get();
        let num_fonts: u32 = self.buffer.get();
        let offset_table: Vec<u32> = self.buffer.get_vec(num_fonts);

        if major_version == 2 {
            let dsig_tag: u32 = self.buffer.get();
            let dsig_length: u32 = self.buffer.get();
            let dsig_offset: u32 = self.buffer.get();
        }

        for offset in offset_table {
            self.buffer.offset = offset as usize;
            self.fonts.push(Font::load_sfnt(&mut self.buffer));
        }
    }

    fn woff_init(&mut self) {
        self.fonts.push(Font::load_woff(&mut self.buffer));
    }

    fn woff2_init(&mut self) {
        self.fonts.push(Font::load_woff2(&mut self.buffer));
    }

    pub fn parse(&mut self) {
        for font in &mut self.fonts {
            match font.format {
                Format::SFNT => font.sfnt_parse(&mut self.buffer),
                Format::WOFF => font.woff_parse(&mut self.buffer),
                Format::WOFF2 => font.woff2_parse(&mut self.buffer),
            }
        }
    }

    // TODO: some tables depend on other tables
    pub fn parse_table(&mut self, tag_str: &str) {
        let tag = &Tag::from(tag_str);
        for font in &mut self.fonts {
            match font.format {
                Format::SFNT => font.sfnt_parse_table(tag, &mut self.buffer),
                Format::WOFF => font.woff_parse_table(tag, &mut self.buffer),
                Format::WOFF2 => font.woff2_parse_table(tag, &mut self.buffer),
            }
        }
    }
}

#[allow(non_snake_case)]
#[derive(Debug, Default)]
#[rustfmt::skip]
pub struct Font {
    format: Format,
    flavor: Flavor,
    table_records: HashMap<Tag, TableRecord>,

    // Required tables

    /// Font header.
    pub head: Option<required::head::Table_head>,
    /// Horizontal header.
    pub hhea: Option<required::hhea::Table_hhea>,
    /// Maximum profile.
    pub maxp: Option<required::maxp::Table_maxp>,
    /// Horizontal metrics.
    pub hmtx: Option<required::hmtx::Table_hmtx>,
    /// Character to glyph mapping.
    pub cmap: Option<required::cmap::Table_cmap>,
    /// Naming table.
    pub name: Option<required::name::Table_name>,
    /// OS/2 and Windows specific metrics.
    pub OS_2: Option<required::os_2::Table_OS_2>,
    /// PostScript information.
    pub post: Option<required::post::Table_post>,

    // Tables related to TrueType outlines

    /// Index to location.
    pub loca: Option<ttf::loca::Table_loca>,
    /// Glyph data.
    pub glyf: Option<ttf::glyf::Table_glyf>,
    /// Control Value Table (optional table).
    pub cvt_: Option<ttf::cvt_::Table_cvt_>,
    /// Font program (optional table).
    pub fpgm: Option<ttf::fpgm::Table_fpgm>,
    /// CVT Program (optional table).
    pub prep: Option<ttf::prep::Table_prep>,
    /// Grid-fitting/Scan-conversion (optional table).
    pub gasp: Option<ttf::gasp::Table_gasp>,

    // Tables Related to CFF Outlines

    /// Compact Font Format 1.0
    pub CFF_: Option<cff::cff_::Table_CFF_>,
    // /// Compact Font Format 2.0
    // pub CFF2: Option<Table_CFF2>,
    /// Vertical Origin (optional table)
    pub VORG: Option<cff::vorg::Table_VORG>,

    // Tables Related to Bitmap Glyphs

    /// Embedded bitmap data
    pub EBDT: Option<bitmap::ebdt::Table_EBDT>,
    /// Embedded bitmap location data
    pub EBLC: Option<bitmap::eblc::Table_EBLC>,
    /// Embedded bitmap scaling data
    pub EBSC: Option<bitmap::ebsc::Table_EBSC>,

    // Advanced Typographic Tables

    /// Baseline data
    pub BASE: Option<layout::base::Table_BASE>,
    // /// Glyph definition data
    // pub GDEF: Option<layout::gdef::Table_GDEF>,
    // /// Glyph positioning data
    // pub GPOS: Option<layout::gpos::Table_GPOS>,
    /// Glyph substitution data
    pub GSUB: Option<layout::gsub::Table_GSUB>,
    /// Justification data
    pub JSTF: Option<layout::jstf::Table_JSTF>,
    /// Math layout data
    pub MATH: Option<layout::math::Table_MATH>,

    // Tables used for OpenType font variations

    /// Axis variations.
    pub avar: Option<otvar::avar::Table_avar>,
    // /// CVT variations (TrueType outlines only)
    // pub cvar: Option<otvar::cvar::Table_cvar>,
    /// Font variations.
    pub fvar: Option<otvar::fvar::Table_fvar>,
    // /// Glyph variations (TrueType outlines only)
    // pub gvar: Option<otvar::gvar::Table_gvar>,
    /// Horizontal metrics variations.
    pub HVAR: Option<otvar::hvar::Table_HVAR>,
    /// Metrics variations.
    pub MVAR: Option<otvar::mvar::Table_MVAR>,
    // /// Style attributes
    // pub STAT: Option<otvar::stat::Table_STAT>,
    // /// Vertical metrics variations
    // pub VVAR: Option<otvar::vvar::Table_VVAR>,

    // Tables Related to Color Fonts

    /// Color table
    pub COLR: Option<color::colr::Table_COLR>,
    /// Color palette table
    pub CPAL: Option<color::cpal::Table_CPAL>,
    /// Color bitmap data
    pub CBDT: Option<color::cbdt::Table_CBDT>,
    /// Color bitmap location data
    pub CBLC: Option<color::cblc::Table_CBLC>,
    /// Standard bitmap graphics
    pub sbix: Option<color::sbix::Table_sbix>,
    /// The SVG (Scalable Vector Graphics) table
    pub SVG_: Option<color::svg_::Table_SVG_>,

    // Other OpenType Tables

    /// Digital signature
    pub DSIG: Option<other::dsig::Table_DSIG>,
    // /// Horizontal device metrics
    // pub hdmx: Option<other::hdmx::Table_hdmx>,
    // /// Kerning
    // pub kern: Option<other::kern::Table_kern>,
    /// Linear threshold data
    pub LTSH: Option<other::ltsh::Table_LTSH>,
    // /// Merge
    // pub MERG: Option<other::merg::Table_MERG>,
    // /// Metadata
    // pub meta: Option<other::meta::Table_meta>,
    // /// PCL 5 data
    // pub PCLT: Option<other::pclt::Table_PCLT>,
    // /// Vertical device metrics
    // pub VDMX: Option<other::vdmx::Table_VDMX>,
    // /// Vertical Metrics header
    // pub vhea: Option<other::vhea::Table_vhea>,
    // /// Vertical Metrics
    // pub vmtx: Option<other::vmtx::Table_vmtx>,
}

impl Font {
    fn load_sfnt(buffer: &mut Buffer) -> Self {
        let signature: u32 = buffer.get();
        let num_tables: u16 = buffer.get();
        let _search_range: u16 = buffer.get();
        let _entry_selector: u16 = buffer.get();
        let _range_shift: u16 = buffer.get();
        Self {
            format: Format::SFNT,
            flavor: Self::get_flavor(signature),
            table_records: (0..num_tables)
                .map(|_| {
                    (
                        buffer.get(),
                        TableRecord {
                            checksum: buffer.get(),
                            offset: buffer.get(),
                            length: buffer.get(),
                            ..Default::default()
                        },
                    )
                })
                .collect(),
            ..Default::default()
        }
    }

    fn load_woff(buffer: &mut Buffer) -> Self {
        let _signature: u32 = buffer.get();
        let flavor: u32 = buffer.get();
        let _length: u32 = buffer.get();
        let num_tables: u16 = buffer.get();
        let _total_sfnt_size: u32 = {
            buffer.skip::<u16>(1);
            buffer.get()
        };
        let _major_version: u16 = buffer.get();
        let _minor_version: u16 = buffer.get();
        let _meta_offset: u32 = buffer.get();
        let _meta_length: u32 = buffer.get();
        let _meta_orig_length: u32 = buffer.get();
        let _priv_offset: u32 = buffer.get();
        let _priv_length: u32 = buffer.get();
        Self {
            format: Format::WOFF,
            flavor: Self::get_flavor(flavor),
            table_records: (0..num_tables)
                .map(|_| {
                    (
                        buffer.get(),
                        TableRecord {
                            // The order is different from SFNT format
                            offset: buffer.get(),
                            woff_comp_length: buffer.get(),
                            length: buffer.get(),
                            checksum: buffer.get(),
                        },
                    )
                })
                .collect(),
            ..Default::default()
        }
    }

    // TODO: WOFF2
    fn load_woff2(buffer: &mut Buffer) -> Self {
        let _signature: u32 = buffer.get();
        let flavor: u32 = buffer.get();
        let _length: u32 = buffer.get();
        let _num_tables: u16 = buffer.get();
        let _total_sfnt_size: u32 = {
            buffer.skip::<u16>(1);
            buffer.get()
        };
        let _total_compressed_size: u32 = buffer.get();
        let _major_version: u16 = buffer.get();
        let _minor_version: u16 = buffer.get();
        let _meta_offset: u32 = buffer.get();
        let _meta_length: u32 = buffer.get();
        let _meta_orig_length: u32 = buffer.get();
        let _priv_offset: u32 = buffer.get();
        let _priv_length: u32 = buffer.get();
        Self {
            format: Format::WOFF2,
            flavor: Self::get_flavor(flavor),
            ..Default::default()
        }
    }

    fn get_flavor(flavor: u32) -> Flavor {
        match flavor {
            SIGNATURE_OTF => Flavor::CFF,
            SIGNATURE_TTF | SIGNATURE_TTF_TRUE | SIGNATURE_TTF_TYP1 => Flavor::TTF,
            _ => unreachable!(),
        }
    }
}

impl Font {
    #[rustfmt::skip]
    fn sfnt_parse(&mut self, buffer: &mut Buffer) {
        for tag_str in &["head", "hhea", "maxp", "hmtx", "cmap", "name", "OS/2", "post"] {
            let tag = &Tag::from(tag_str);
            self.sfnt_parse_table(tag, buffer);
        }
        for tag_str in &[
            "loca", "glyf", "cvt ", "fpgm", "prep", "gasp",
            "CFF ", "VORG",
            "BASE", "GSUB", "JSTF", "MATH",
            "EBLC", "EBDT", "EBSC",
            "CBLC", "CBDT",
            "COLR", "CPAL",
            "sbix",
            "SVG ",
            "DSIG", "LTSH"
        ] {
            let tag = &Tag::from(tag_str);
            if self.table_records.contains_key(tag) {
                self.sfnt_parse_table(tag, buffer);
            }
        }
    }

    fn sfnt_parse_table(&mut self, tag: &Tag, buffer: &mut Buffer) {
        buffer.offset = self.get_table_offset(tag);
        self._parse_table(tag, buffer);
    }

    fn woff_parse(&mut self, buffer: &mut Buffer) {
        for tag_str in &[
            "head", "hhea", "maxp", "hmtx", "cmap", "name", "OS/2", "post",
        ] {
            let tag = &Tag::from(tag_str);
            self.woff_parse_table(tag, buffer);
        }
        for tag_str in &[
            "loca", "glyf", "cvt ", "fpgm", "prep", "gasp", "CFF ", "CFF2",
        ] {
            let tag = &Tag::from(tag_str);
            if self.table_records.contains_key(tag) {
                self.woff_parse_table(tag, buffer);
            }
        }
    }

    fn woff_parse_table(&mut self, tag: &Tag, buffer: &mut Buffer) {
        buffer.offset = self.get_table_offset(tag);
        let comp_length = self.get_table_comp_len(tag);
        self._parse_table(tag, &mut buffer.zlib_decompress(comp_length));
    }

    #[allow(unused_variables)]
    fn woff2_parse(&mut self, buffer: &mut Buffer) {
        unimplemented!()
    }

    #[allow(unused_variables)]
    fn woff2_parse_table(&mut self, tag: &Tag, buffer: &mut Buffer) {
        unimplemented!()
    }

    fn _parse_table(&mut self, tag: &Tag, buffer: &mut Buffer) {
        match tag.to_str() {
            "head" => self.parse_head(buffer),
            "hhea" => self.parse_hhea(buffer),
            "maxp" => self.parse_maxp(buffer),
            "hmtx" => self.parse_hmtx(buffer),
            "cmap" => self.parse_cmap(buffer),
            "name" => self.parse_name(buffer),
            "OS/2" => self.parse_OS_2(buffer),
            "post" => self.parse_post(buffer),
            "loca" => self.parse_loca(buffer),
            "glyf" => self.parse_glyf(buffer),
            "cvt " => self.parse_cvt_(buffer),
            "fpgm" => self.parse_fpgm(buffer),
            "prep" => self.parse_prep(buffer),
            "gasp" => self.parse_gasp(buffer),
            "CFF " => self.parse_CFF_(buffer),
            // "CFF2" => self.parse_CFF2(buffer),
            "VORG" => self.parse_VORG(buffer),
            "EBDT" => self.parse_EBDT(buffer),
            "EBLC" => self.parse_EBLC(buffer),
            "EBSC" => self.parse_EBSC(buffer),
            "BASE" => self.parse_BASE(buffer),
            "GSUB" => self.parse_GSUB(buffer),
            "JSTF" => self.parse_JSTF(buffer),
            "MATH" => self.parse_MATH(buffer),
            "avar" => self.parse_avar(buffer),
            // "cvar" => self.parse_cvar(buffer),
            "fvar" => self.parse_fvar(buffer),
            // "gvar" => self.parse_gvar(buffer),
            "HVAR" => self.parse_HVAR(buffer),
            "MVAR" => self.parse_MVAR(buffer),
            // "STAT" => self.parse_STAT(buffer),
            // "VVAR" => self.parse_VVAR(buffer),
            "COLR" => self.parse_COLR(buffer),
            "CPAL" => self.parse_CPAL(buffer),
            "CBDT" => self.parse_CBDT(buffer),
            "CBLC" => self.parse_CBLC(buffer),
            "sbix" => self.parse_sbix(buffer),
            "SVG " => self.parse_SVG_(buffer),
            "DSIG" => self.parse_DSIG(buffer),
            "LTSH" => self.parse_LTSH(buffer),
            _ => eprintln!("Table `{}` is not supported", tag),
        };
    }

    // TODO: consider Option<>

    fn _get_table_record(&self, tag: &Tag) -> &TableRecord {
        self.table_records.get(tag).unwrap()
    }

    pub fn get_table_len(&self, tag: &Tag) -> usize {
        self._get_table_record(tag).length as usize
    }

    pub fn get_table_offset(&self, tag: &Tag) -> usize {
        self._get_table_record(tag).offset as usize
    }

    pub fn get_table_comp_len(&self, tag: &Tag) -> usize {
        self._get_table_record(tag).woff_comp_length as usize
    }

    pub fn has_table(&self, tag_str: &str) -> bool {
        self.table_records.contains_key(&Tag::from(tag_str))
    }
}

#[derive(Debug)]
enum Format {
    SFNT,
    WOFF,
    WOFF2,
}

impl Default for Format {
    fn default() -> Self {
        Self::SFNT
    }
}

#[derive(Debug)]
enum Flavor {
    TTF,
    CFF,
}

impl Default for Flavor {
    fn default() -> Self {
        Self::TTF
    }
}

#[derive(Debug, Default)]
struct TableRecord {
    checksum: u32,
    offset: u32,
    length: u32,
    woff_comp_length: u32,
}

/// For OpenType fonts containing CFF data (version 1 or 2), which is `OTTO`.
const SIGNATURE_OTF: u32 = 0x4F54_544F;
/// For OpenType fonts that contain TrueType outlines.
const SIGNATURE_TTF: u32 = 0x0001_0000;
/// The Apple specification for TrueType fonts allows for `true`. Should NOT be used.
const SIGNATURE_TTF_TRUE: u32 = 0x7472_7565;
/// The Apple specification for TrueType fonts allows for `typ1`. Should NOT be used.
const SIGNATURE_TTF_TYP1: u32 = 0x7479_7031;
/// Font Collection ID string: `ttcf`.
const SIGNATURE_TTC: u32 = 0x7474_6366;
/// The `signature` field in the WOFF (version 1) header MUST contain this "magic number" `wOFF`.
const SIGNATURE_WOFF: u32 = 0x774F_4646;
/// The `signature` field in the WOFF (version 2) header MUST contain this "magic number" `wOF2`.
const SIGNATURE_WOFF2: u32 = 0x774F_4632;
