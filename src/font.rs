use crate::tables::*;
use crate::types::Tag;
use crate::util::Buffer;
use std::fs;
use std::io;
use std::iter::{FromIterator, Zip};
use std::slice::Iter;

/// The container for a [OpenType]/[WOFF]/[WOFF2] font or font collection.
///
/// [OpenType]: https://docs.microsoft.com/en-us/typography/opentype/spec/
/// [WOFF]: https://www.w3.org/TR/WOFF/
/// [WOFF2]: https://www.w3.org/TR/WOFF2/
#[derive(Debug)]
pub struct FontContainer {
    buffer: Buffer,
    pub fonts: Vec<Font>,
}

impl FontContainer {
    /// Font Collection ID string: `ttcf`.
    const SIGNATURE_TTC: u32 = 0x7474_6366;
    /// The `signature` field in the WOFF (version 1) header MUST contain this "magic number" `wOFF`.
    const SIGNATURE_WOFF: u32 = 0x774F_4646;
    /// The `signature` field in the WOFF (version 2) header MUST contain this "magic number" `wOF2`.
    const SIGNATURE_WOFF2: u32 = 0x774F_4632;

    /// Read and initializes a font container from a file.
    ///
    /// # Errors
    ///
    /// This function will return an error if `path` does not already exist.
    /// Other errors may also be returned according to [`fs::read`].
    ///
    /// # Examples
    ///
    /// ```ignore
    /// use rustotf;
    ///
    /// fn main() -> std::io::Result<()> {
    ///     let font = rustotf::FontContainer::read("font.ttf")?;
    ///     Ok(())
    /// }
    /// ```
    pub fn read(path: &str) -> io::Result<Self> {
        let bytes = fs::read(path)?;
        let mut font_container = Self::new(bytes);
        font_container.init();
        Ok(font_container)
    }

    /// Create an empty font container.
    fn new(bytes: Vec<u8>) -> Self {
        Self {
            buffer: Buffer::new(bytes),
            fonts: Vec::new(),
        }
    }

    /// Initialize the font container.
    fn init(&mut self) {
        let signature = self.buffer.get();
        self.buffer.set_offset(0);
        match signature {
            Self::SIGNATURE_TTC => self.init_ttc(),
            Self::SIGNATURE_WOFF => self.init_woff(),
            Self::SIGNATURE_WOFF2 => self.init_woff2(),
            _ => self.init_otf(),
        }
    }

    fn init_otf(&mut self) {
        self.fonts.push(Font::load_sfnt(&mut self.buffer));
    }

    #[allow(unused_variables)]
    fn init_ttc(&mut self) {
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
            self.buffer.set_offset(offset);
            self.fonts.push(Font::load_sfnt(&mut self.buffer));
        }
    }

    fn init_woff(&mut self) {
        self.fonts.push(Font::load_woff(&mut self.buffer));
    }

    fn init_woff2(&mut self) {
        self.fonts.push(Font::load_woff2(&mut self.buffer));
    }

    /// Parse all the tables in each font of the container.
    pub fn parse(&mut self) {
        let buffer = &mut self.buffer;
        self.fonts.iter_mut().for_each(|font| font.parse(buffer));
    }

    /// Parse all the tables in the font at `index` of the container.
    pub fn parse_nth(&mut self, index: usize) {
        match self.fonts.get_mut(index) {
            Some(font) => font.parse(&mut self.buffer),
            None => panic!(),
        }
    }

    /// Parse the table with `tag` in each font of the container.
    pub fn parse_table(&mut self, tag: Tag) {
        // TODO: some tables depend on other tables
        let buffer = &mut self.buffer;
        self.fonts
            .iter_mut()
            .for_each(|font| font.parse_table(tag, buffer));
    }

    /// Parse the table with `tag` in the font at `index` of the container.
    pub fn parse_table_nth(&mut self, tag: Tag, index: usize) {
        match self.fonts.get_mut(index) {
            Some(font) => font.parse_table(tag, &mut self.buffer),
            None => panic!(),
        }
    }
}

#[allow(non_snake_case)]
#[derive(Debug, Default)]
#[rustfmt::skip]
pub struct Font {
    format: Format,
    flavor: Flavor,
    table_records: TableRecords,

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
        // Skip searchRange, entrySelector and rangeShift.
        buffer.skip::<u16>(3);
        let table_records = (0..num_tables)
            .map(|_| {
                let tag = buffer.get();
                let record = TableRecord {
                    checksum: buffer.get(),
                    offset: buffer.get(),
                    length: buffer.get(),
                    ..Default::default()
                };
                (tag, record)
            })
            .collect();
        Self {
            format: Format::SFNT,
            flavor: Flavor::from(signature),
            table_records,
            ..Default::default()
        }
    }

    #[allow(unused_variables)]
    fn load_woff(buffer: &mut Buffer) -> Self {
        let signature: u32 = buffer.get();
        let flavor: u32 = buffer.get();
        let length: u32 = buffer.get();
        let num_tables: u16 = buffer.get();
        let total_sfnt_size: u32 = {
            buffer.skip::<u16>(1);
            buffer.get()
        };
        let major_version: u16 = buffer.get();
        let minor_version: u16 = buffer.get();
        let meta_offset: u32 = buffer.get();
        let meta_length: u32 = buffer.get();
        let meta_orig_length: u32 = buffer.get();
        let priv_offset: u32 = buffer.get();
        let priv_length: u32 = buffer.get();
        let table_records = (0..num_tables)
            .map(|_| {
                let tag = buffer.get();
                let record = TableRecord {
                    // The order is different from SFNT format
                    offset: buffer.get(),
                    woff_comp_length: buffer.get(),
                    length: buffer.get(),
                    checksum: buffer.get(),
                };
                (tag, record)
            })
            .collect();
        Self {
            format: Format::WOFF,
            flavor: Flavor::from(flavor),
            table_records,
            ..Default::default()
        }
    }

    // TODO: WOFF2
    #[allow(unused_variables)]
    fn load_woff2(buffer: &mut Buffer) -> Self {
        let signature: u32 = buffer.get();
        let flavor: u32 = buffer.get();
        let length: u32 = buffer.get();
        let num_tables: u16 = buffer.get();
        let total_sfnt_size: u32 = {
            buffer.skip::<u16>(1);
            buffer.get()
        };
        let total_compressed_size: u32 = buffer.get();
        let major_version: u16 = buffer.get();
        let minor_version: u16 = buffer.get();
        let meta_offset: u32 = buffer.get();
        let meta_length: u32 = buffer.get();
        let meta_orig_length: u32 = buffer.get();
        let priv_offset: u32 = buffer.get();
        let priv_length: u32 = buffer.get();
        Self {
            format: Format::WOFF2,
            flavor: Flavor::from(flavor),
            ..Default::default()
        }
    }

    pub fn parse(&mut self, buffer: &mut Buffer) {
        match self.format {
            Format::SFNT => self.parse_sfnt(buffer),
            Format::WOFF => self.parse_woff(buffer),
            Format::WOFF2 => self.parse_woff2(buffer),
        }
    }

    pub fn parse_table(&mut self, tag: Tag, buffer: &mut Buffer) {
        match self.format {
            Format::SFNT => self.parse_sfnt_table(tag, buffer),
            Format::WOFF => self.parse_woff_table(tag, buffer),
            Format::WOFF2 => self.parse_woff2_table(tag, buffer),
        }
    }

    fn parse_sfnt(&mut self, buffer: &mut Buffer) {
        let required_tables = &[
            b"head", b"hhea", b"maxp", b"hmtx", b"cmap", b"name", b"OS/2", b"post",
        ];
        let tables = &[
            b"loca", b"glyf", b"cvt ", b"fpgm", b"prep", b"gasp", // TrueType
            b"CFF ", b"VORG", // CFF
            b"BASE", b"GSUB", b"JSTF", b"MATH", // OpenType layout
            b"EBLC", b"EBDT", b"EBSC", // Bitmap
            b"CBLC", b"CBDT", b"COLR", b"CPAL", b"sbix", b"SVG ", // Color
            b"DSIG", b"LTSH", // Other
        ];

        for tag_str in required_tables {
            let tag = Tag::new(tag_str);
            self.parse_sfnt_table(tag, buffer);
        }
        for tag_str in tables {
            let tag = Tag::new(tag_str);
            if self.table_records.contains(&tag) {
                self.parse_sfnt_table(tag, buffer);
            }
        }
    }

    fn parse_sfnt_table(&mut self, tag: Tag, buffer: &mut Buffer) {
        buffer.set_offset(self.get_table_offset(tag));
        self.parse_table_internal(tag, buffer);
    }

    fn parse_woff(&mut self, buffer: &mut Buffer) {
        let required_tables = &[
            b"head", b"hhea", b"maxp", b"hmtx", b"cmap", b"name", b"OS/2", b"post",
        ];
        let tables = &[
            b"loca", b"glyf", b"cvt ", b"fpgm", b"prep", b"gasp", // TrueType
            b"CFF ", b"VORG", // CFF
            b"BASE", b"GSUB", b"JSTF", b"MATH", // OpenType layout
            b"EBLC", b"EBDT", b"EBSC", // Bitmap
            b"CBLC", b"CBDT", b"COLR", b"CPAL", b"sbix", b"SVG ", // Color
            b"DSIG", b"LTSH", // Other
        ];

        for tag_str in required_tables {
            let tag = Tag::new(tag_str);
            self.parse_woff_table(tag, buffer);
        }
        for tag_str in tables {
            let tag = Tag::new(tag_str);
            if self.table_records.contains(&tag) {
                self.parse_woff_table(tag, buffer);
            }
        }
    }

    fn parse_woff_table(&mut self, tag: Tag, buffer: &mut Buffer) {
        buffer.set_offset(self.get_table_offset(tag));
        let len = self.get_table_len(tag);
        let comp_len = self.get_table_comp_len(tag);
        if comp_len < len {
            match &mut buffer.zlib_decompress(comp_len) {
                Ok(orig_buffer) => self.parse_table_internal(tag, orig_buffer),
                Err(_) => panic!(),
            }
        } else {
            self.parse_table_internal(tag, buffer);
        }
    }

    #[allow(unused_variables)]
    fn parse_woff2(&mut self, buffer: &mut Buffer) {
        unimplemented!()
    }

    #[allow(unused_variables)]
    fn parse_woff2_table(&mut self, tag: Tag, buffer: &mut Buffer) {
        unimplemented!()
    }

    fn parse_table_internal(&mut self, tag: Tag, buffer: &mut Buffer) {
        match tag.bytes() {
            b"head" => self.parse_head(buffer),
            b"hhea" => self.parse_hhea(buffer),
            b"maxp" => self.parse_maxp(buffer),
            b"hmtx" => self.parse_hmtx(buffer),
            b"cmap" => self.parse_cmap(buffer),
            b"name" => self.parse_name(buffer),
            b"OS/2" => self.parse_OS_2(buffer),
            b"post" => self.parse_post(buffer),
            b"loca" => self.parse_loca(buffer),
            b"glyf" => self.parse_glyf(buffer),
            b"cvt " => self.parse_cvt_(buffer),
            b"fpgm" => self.parse_fpgm(buffer),
            b"prep" => self.parse_prep(buffer),
            b"gasp" => self.parse_gasp(buffer),
            b"CFF " => self.parse_CFF_(buffer),
            // b"CFF2" => self.parse_CFF2(buffer),
            b"VORG" => self.parse_VORG(buffer),
            b"EBDT" => self.parse_EBDT(buffer),
            b"EBLC" => self.parse_EBLC(buffer),
            b"EBSC" => self.parse_EBSC(buffer),
            b"BASE" => self.parse_BASE(buffer),
            b"GSUB" => self.parse_GSUB(buffer),
            b"JSTF" => self.parse_JSTF(buffer),
            b"MATH" => self.parse_MATH(buffer),
            b"avar" => self.parse_avar(buffer),
            // b"cvar" => self.parse_cvar(buffer),
            b"fvar" => self.parse_fvar(buffer),
            // b"gvar" => self.parse_gvar(buffer),
            b"HVAR" => self.parse_HVAR(buffer),
            b"MVAR" => self.parse_MVAR(buffer),
            // b"STAT" => self.parse_STAT(buffer),
            // b"VVAR" => self.parse_VVAR(buffer),
            b"COLR" => self.parse_COLR(buffer),
            b"CPAL" => self.parse_CPAL(buffer),
            b"CBDT" => self.parse_CBDT(buffer),
            b"CBLC" => self.parse_CBLC(buffer),
            b"sbix" => self.parse_sbix(buffer),
            b"SVG " => self.parse_SVG_(buffer),
            b"DSIG" => self.parse_DSIG(buffer),
            b"LTSH" => self.parse_LTSH(buffer),
            _ => eprintln!("Table `{}` is not supported", tag),
        };
    }

    // TODO: consider Option<>

    fn get(&self, tag: Tag) -> &TableRecord {
        self.table_records.get(tag).unwrap()
    }

    pub fn get_table_len(&self, tag: Tag) -> usize {
        self.get(tag).length as usize
    }

    pub fn get_table_offset(&self, tag: Tag) -> usize {
        self.get(tag).offset as usize
    }

    pub fn get_table_comp_len(&self, tag: Tag) -> usize {
        self.get(tag).woff_comp_length as usize
    }

    pub fn contains(&self, s: &str) -> bool {
        self.table_records.contains(&Tag::from(s))
    }

    pub fn fmt_tables(&self, indent: &str) -> String {
        #[rustfmt::skip]
        let header = format!(
            concat!(
                "{0}", "Tag     Checksum      Length      Offset", "\n",
                "{0}", "----  ----------  ----------  ----------", "\n",
            ),
            indent
        );
        let body = self
            .table_records
            .into_iter()
            .map(|(tag, rec)| {
                format!(
                    "{}{}  0x{:08X}  {:10}  {:10}",
                    indent, tag, rec.checksum, rec.length, rec.offset
                )
            })
            .collect::<Vec<_>>()
            .join("\n");
        format!("{}{}", header, body)
    }
}

#[derive(Debug, Default)]
struct TableRecords {
    tags: Vec<Tag>,
    records: Vec<TableRecord>,
}

impl TableRecords {
    fn get(&self, tag: Tag) -> Option<&TableRecord> {
        match self.tags.iter().position(|&t| t == tag) {
            Some(pos) => Some(&self.records[pos]),
            _ => None,
        }
    }

    fn contains(&self, tag: &Tag) -> bool {
        self.tags.contains(tag)
    }
}

impl<'a> IntoIterator for &'a TableRecords {
    type Item = (&'a Tag, &'a TableRecord);
    type IntoIter = Zip<Iter<'a, Tag>, Iter<'a, TableRecord>>;

    fn into_iter(self) -> Self::IntoIter {
        self.tags
            .as_slice()
            .iter()
            .zip(self.records.as_slice().iter())
    }
}

impl FromIterator<(Tag, TableRecord)> for TableRecords {
    fn from_iter<T>(iter: T) -> Self
    where
        T: IntoIterator<Item = (Tag, TableRecord)>,
    {
        let mut res = TableRecords::default();
        for (tag, record) in iter {
            res.tags.push(tag);
            res.records.push(record);
        }
        res
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

impl Flavor {
    /// For OpenType fonts containing CFF data (version 1 or 2), which is `OTTO`.
    const SIGNATURE_OTF: u32 = 0x4F54_544F;
    /// For OpenType fonts that contain TrueType outlines.
    const SIGNATURE_TTF: u32 = 0x0001_0000;
    /// The Apple specification for TrueType fonts allows for `true`. Should NOT be used.
    const SIGNATURE_TTF_TRUE: u32 = 0x7472_7565;
    /// The Apple specification for TrueType fonts allows for `typ1`. Should NOT be used.
    const SIGNATURE_TTF_TYP1: u32 = 0x7479_7031;

    fn from(flavor: u32) -> Self {
        match flavor {
            Self::SIGNATURE_OTF => Self::CFF,
            Self::SIGNATURE_TTF | Self::SIGNATURE_TTF_TRUE | Self::SIGNATURE_TTF_TYP1 => Self::TTF,
            _ => unreachable!(),
        }
    }
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
