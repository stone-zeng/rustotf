use crate::tables::*;
use crate::types::{u32_var, Tag};
use crate::util::{Buffer, ReadBuffer};
use std::fs;
use std::io;
use std::iter::{FromIterator, Zip};
use std::slice::Iter;

/// The container of a OpenType/WOFF/WOFF2 font or font collection.
///
/// For an [OpenType] font (`.ttf` or `.otf`), the container is consist of a single [`Font`]
/// object. The [OpenType Font Collection] (`.ttc` or `.otc`) is also allowed, where multiple
/// OpenType font resources are stored in a single container.
///
/// The above fonts and font collections are stored in the so called SFNT container format.
/// For web use, SFNT fonts are repackaged into the [WOFF] or [WOFF2] formats to reduce file size.
/// Note that WOFF 1.0 does not support font collections.
///
/// [OpenType]: https://docs.microsoft.com/en-us/typography/opentype/spec/
/// [OpenType Font Collection]: https://docs.microsoft.com/en-us/typography/opentype/spec/otff#font-collections
/// [WOFF]: https://www.w3.org/TR/WOFF/
/// [WOFF2]: https://www.w3.org/TR/WOFF2/
#[derive(Debug)]
pub struct FontContainer {
    buffer: Buffer,
    fonts: Vec<Font>,
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
    ///     let font = rustotf::FontContainer::read("SourceSerif4-Regular.otf")?;
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
        let ttc_tag: u32 = self.buffer.get(); // "ttcf"
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

    #[allow(unused_variables)]
    fn init_woff2(&mut self) {
        let signature: u32 = self.buffer.get();
        let flavor: u32 = self.buffer.get();
        match flavor {
            // TODO: WOFF2 collections
            Self::SIGNATURE_TTC => unimplemented!(),
            _ => {
                self.buffer.set_offset(0);
                self.fonts.push(Font::load_woff2(&mut self.buffer));
            }
        }
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

    /// Return the number of [`Font`] objects in the container.
    pub fn len(&self) -> usize {
        self.fonts.len()
    }

    /// Return `true` if the font container has no fonts.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Return a reference to a [`Font`] object at given position, or `None` if out of bounds.
    pub fn get(&self, pos: usize) -> Option<&Font> {
        self.fonts.get(pos)
    }
}

impl<'a> IntoIterator for &'a FontContainer {
    type Item = &'a Font;
    type IntoIter = Iter<'a, Font>;

    fn into_iter(self) -> Self::IntoIter {
        self.fonts.iter()
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
            format: Format::Sfnt,
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
                    comp_length: buffer.get(),
                    length: buffer.get(),
                    checksum: buffer.get(),
                };
                (tag, record)
            })
            .collect();
        Self {
            format: Format::Woff,
            flavor: Flavor::from(flavor),
            table_records,
            ..Default::default()
        }
    }

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
        let table_entries: Vec<Woff2TableEntry> = buffer.get_vec(num_tables);
        let table_records = table_entries
            .iter()
            .map(|entry| {
                let tag = entry.tag;
                // TODO: checksum and offset in WOFF2
                let record = TableRecord {
                    checksum: 0,
                    offset: 0,
                    length: entry.orig_len,
                    comp_length: entry.transform_len,
                };
                (tag, record)
            })
            .collect();
        Self {
            format: Format::Woff2,
            flavor: Flavor::from(flavor),
            table_records,
            ..Default::default()
        }
    }

    pub fn parse(&mut self, buffer: &mut Buffer) {
        match self.format {
            Format::Sfnt => self.parse_sfnt(buffer),
            Format::Woff => self.parse_woff(buffer),
            Format::Woff2 => self.parse_woff2(buffer),
        }
    }

    pub fn parse_table(&mut self, tag: Tag, buffer: &mut Buffer) {
        match self.format {
            Format::Sfnt => self.parse_sfnt_table(tag, buffer),
            Format::Woff => self.parse_woff_table(tag, buffer),
            Format::Woff2 => self.parse_woff2_table(tag, buffer),
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
        self.get(tag).comp_length as usize
    }

    pub fn contains(&self, s: &str) -> bool {
        self.table_records.contains(&Tag::from(s))
    }

    pub fn fmt_font_info(&self, indent: &str) -> String {
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

    pub fn fmt_tables(&self, tables: &[&str]) -> String {
        match tables.len() {
            0 => self
                .table_records
                .into_iter()
                .map(|(&tag, _)| self.fmt_table(tag))
                .collect(),
            _ => tables
                .iter()
                .map(|&s| self.fmt_table(Tag::from(s)))
                .collect(),
        }
    }

    fn fmt_table(&self, tag: Tag) -> String {
        macro_rules! fmt {
            ($table:ident) => {{
                match &self.$table {
                    Some(t) => format!("{:#?}\n", t),
                    _ => "".to_string(),
                }
            }};
        }
        match tag.bytes() {
            b"head" => fmt!(head),
            b"hhea" => fmt!(hhea),
            b"maxp" => fmt!(maxp),
            b"hmtx" => fmt!(hmtx),
            b"cmap" => fmt!(cmap),
            b"name" => fmt!(name),
            b"OS/2" => fmt!(OS_2),
            b"post" => fmt!(post),
            b"loca" => fmt!(loca),
            b"glyf" => fmt!(glyf),
            b"cvt " => fmt!(cvt_),
            b"fpgm" => fmt!(fpgm),
            b"prep" => fmt!(prep),
            b"gasp" => fmt!(gasp),
            b"CFF " => fmt!(CFF_),
            // b"CFF2" => fmt!(CFF2),
            b"VORG" => fmt!(VORG),
            b"EBDT" => fmt!(EBDT),
            b"EBLC" => fmt!(EBLC),
            b"EBSC" => fmt!(EBSC),
            b"BASE" => fmt!(BASE),
            b"GSUB" => fmt!(GSUB),
            b"JSTF" => fmt!(JSTF),
            b"MATH" => fmt!(MATH),
            b"avar" => fmt!(avar),
            // b"cvar" => fmt!(cvar),
            b"fvar" => fmt!(fvar),
            // b"gvar" => fmt!(gvar),
            b"HVAR" => fmt!(HVAR),
            b"MVAR" => fmt!(MVAR),
            // b"STAT" => fmt!(STAT),
            // b"VVAR" => fmt!(VVAR),
            b"COLR" => fmt!(COLR),
            b"CPAL" => fmt!(CPAL),
            b"CBDT" => fmt!(CBDT),
            b"CBLC" => fmt!(CBLC),
            b"sbix" => fmt!(sbix),
            b"SVG " => fmt!(SVG_),
            b"DSIG" => fmt!(DSIG),
            b"LTSH" => fmt!(LTSH),
            _ => {
                eprintln!("Table `{}` is not supported", tag);
                "".to_string()
            }
        }
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
    Sfnt,
    Woff,
    Woff2,
}

impl Default for Format {
    fn default() -> Self {
        Self::Sfnt
    }
}

#[derive(Debug)]
enum Flavor {
    Ttf,
    Cff,
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
            Self::SIGNATURE_OTF => Self::Cff,
            Self::SIGNATURE_TTF | Self::SIGNATURE_TTF_TRUE | Self::SIGNATURE_TTF_TYP1 => Self::Ttf,
            _ => unreachable!(),
        }
    }
}

impl Default for Flavor {
    fn default() -> Self {
        Self::Ttf
    }
}

#[derive(Debug, Default)]
struct TableRecord {
    checksum: u32,
    offset: u32,
    length: u32,
    comp_length: u32,
}

// TODO:
#[allow(dead_code)]
struct Woff2TableEntry {
    tag: Tag,
    flags: u8,
    orig_len: u32,
    transform_len: u32,
}

impl Woff2TableEntry {
    fn to_tag(flag: u8) -> Tag {
        match flag {
            0 => Tag::new(b"cmap"),
            1 => Tag::new(b"head"),
            2 => Tag::new(b"hhea"),
            3 => Tag::new(b"hmtx"),
            4 => Tag::new(b"maxp"),
            5 => Tag::new(b"name"),
            6 => Tag::new(b"OS/2"),
            7 => Tag::new(b"post"),
            8 => Tag::new(b"cvt "),
            9 => Tag::new(b"fpgm"),
            10 => Tag::new(b"glyf"),
            11 => Tag::new(b"loca"),
            12 => Tag::new(b"prep"),
            13 => Tag::new(b"CFF "),
            14 => Tag::new(b"VORG"),
            15 => Tag::new(b"EBDT"),
            16 => Tag::new(b"EBLC"),
            17 => Tag::new(b"gasp"),
            18 => Tag::new(b"hdmx"),
            19 => Tag::new(b"kern"),
            20 => Tag::new(b"LTSH"),
            21 => Tag::new(b"PCLT"),
            22 => Tag::new(b"VDMX"),
            23 => Tag::new(b"vhea"),
            24 => Tag::new(b"vmtx"),
            25 => Tag::new(b"BASE"),
            26 => Tag::new(b"GDEF"),
            27 => Tag::new(b"GPOS"),
            28 => Tag::new(b"GSUB"),
            29 => Tag::new(b"EBSC"),
            30 => Tag::new(b"JSTF"),
            31 => Tag::new(b"MATH"),
            32 => Tag::new(b"CBDT"),
            33 => Tag::new(b"CBLC"),
            34 => Tag::new(b"COLR"),
            35 => Tag::new(b"CPAL"),
            36 => Tag::new(b"SVG "),
            37 => Tag::new(b"sbix"),
            38 => Tag::new(b"acnt"),
            39 => Tag::new(b"avar"),
            40 => Tag::new(b"bdat"),
            41 => Tag::new(b"bloc"),
            42 => Tag::new(b"bsln"),
            43 => Tag::new(b"cvar"),
            44 => Tag::new(b"fdsc"),
            45 => Tag::new(b"feat"),
            46 => Tag::new(b"fmtx"),
            47 => Tag::new(b"fvar"),
            48 => Tag::new(b"gvar"),
            49 => Tag::new(b"hsty"),
            50 => Tag::new(b"just"),
            51 => Tag::new(b"lcar"),
            52 => Tag::new(b"mort"),
            53 => Tag::new(b"morx"),
            54 => Tag::new(b"opbd"),
            55 => Tag::new(b"prop"),
            56 => Tag::new(b"trak"),
            57 => Tag::new(b"Zapf"),
            58 => Tag::new(b"Silf"),
            59 => Tag::new(b"Glat"),
            60 => Tag::new(b"Gloc"),
            61 => Tag::new(b"Feat"),
            62 => Tag::new(b"Sill"),
            _ => unreachable!(),
        }
    }
}

impl ReadBuffer for Woff2TableEntry {
    fn read(buffer: &mut Buffer) -> Self {
        let flags = buffer.get();
        let table_flag = flags & 0x3F;
        let trans_version: u8 = flags >> 6;
        let tag = match table_flag {
            0..=62 => Self::to_tag(table_flag),
            _ => {
                let raw_tag: u32 = buffer.get();
                Tag::from(raw_tag)
            }
        };
        let orig_len: u32_var = buffer.get();
        let transform_len: u32_var = if tag == b"glyf" || tag == b"loca" {
            if trans_version == 3 {
                orig_len
            } else {
                buffer.get()
            }
        } else {
            if trans_version == 0 {
                orig_len
            } else {
                buffer.get()
            }
        };
        Self {
            tag,
            flags,
            orig_len: orig_len.into(),
            transform_len: transform_len.into(),
        }
    }
}
