use crate::table::{
    cmap::Table_cmap,
    head::Table_head,
    hhea::Table_hhea,
    hmtx::Table_hmtx,
    maxp::Table_maxp,
    name::Table_name,
    os_2::Table_OS_2,
    post::Table_post,
    avar::Table_avar,
    fvar::Table_fvar,
};
use crate::util::{Buffer, ReadBuffer, Tag};

use std::collections::HashMap;
use std::error::Error;
use std::fs;

pub fn read_font(font_file_path: &str) -> Result<(), Box<dyn Error>> {
    println!("{:?}", font_file_path);

    let mut font_container = FontContainer::new(fs::read(font_file_path)?);
    font_container.init();
    // font_container.parse();

    // font_container.parse_table("fvar");
    font_container.parse_table("avar");

    println!("{:#?}", font_container);
    Ok(())

    // TODO: check extension.
    // let font_file_path = Path::new(font_file_path);
    // let ext = font_file_path.extension().unwrap().to_str().unwrap().to_lowercase();
    // let ext = ext.as_str();
    // match ext {
    //     "otf" | "ttf" => fonts.push(Font::load(raw_buffer)),
    //     "ttc" | "otc" => fonts = FontCollection::load(raw_buffer).fonts,
    //     _ => println!("Not valid extension: {}", ext),
    // };
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
        let signature = self.buffer.get::<u32>();
        self.buffer.offset = 0;
        match signature {
            SIGNATURE_OTF | SIGNATURE_TTF | SIGNATURE_TTF_TRUE | SIGNATURE_TTF_TYP1 => {
                self.init_otf()
            }
            SIGNATURE_TTC => self.init_ttc(),
            SIGNATURE_WOFF => self.init_woff(),
            SIGNATURE_WOFF2 => self.init_woff2(),
            _ => (),
        }
    }

    fn init_otf(&mut self) {
        self.fonts.push(Font::load_sfnt(&mut self.buffer));
    }

    #[allow(unused_variables)]
    fn init_ttc(&mut self) {
        let ttc_tag = self.buffer.get::<u32>();
        let major_version = self.buffer.get::<u16>();
        let minor_version = self.buffer.get::<u16>();
        let num_fonts = self.buffer.get::<u32>() as usize;
        let offset_table = self.buffer.get_vec::<u32>(num_fonts);

        if major_version == 2 {
            let dsig_tag = self.buffer.get::<u32>();
            let dsig_length = self.buffer.get::<u32>();
            let dsig_offset = self.buffer.get::<u32>();
        }

        for offset in offset_table {
            self.buffer.offset = offset as usize;
            self.fonts.push(Font::load_sfnt(&mut self.buffer));
        }
    }

    fn init_woff(&mut self) {
        self.fonts.push(Font::load_woff(&mut self.buffer));
    }

    fn init_woff2(&mut self) {
        self.fonts.push(Font::load_woff2(&mut self.buffer));
    }

    pub fn parse(&mut self) {
        for font in &mut self.fonts {
            match font.format {
                Format::SFNT => font.parse_sfnt(&mut self.buffer),
                Format::WOFF => font.parse_woff(&mut self.buffer),
                Format::WOFF2 => font.parse_woff2(&mut self.buffer),
            }
        }
    }

    pub fn parse_table(&mut self, tag_str: &str) {
        for font in &mut self.fonts {
            match font.format {
                Format::SFNT => font.parse_table_sfnt(tag_str, &mut self.buffer),
                Format::WOFF => font.parse_table_woff(tag_str, &mut self.buffer),
                Format::WOFF2 => font.parse_table_woff2(tag_str, &mut self.buffer),
            }
        }
    }
}

#[allow(non_snake_case)]
#[derive(Debug)]
pub struct Font {
    format: Format,
    flavor: Flavor,
    table_records: HashMap<Tag, TableRecord>,
    // woff_table_records: HashMap<String, WoffTableRecord>,

    // Required Tables
    pub head: Option<Table_head>, // Font header
    pub hhea: Option<Table_hhea>, // Horizontal header
    pub maxp: Option<Table_maxp>, // Maximum profile
    pub hmtx: Option<Table_hmtx>, // Horizontal metrics
    pub cmap: Option<Table_cmap>, // Character to glyph mapping
    pub name: Option<Table_name>, // Naming table
    pub OS_2: Option<Table_OS_2>, // OS/2 and Windows specific metrics
    pub post: Option<Table_post>, // PostScript information

    // Tables used for OpenType Font Variations
    pub avar: Option<Table_avar>, // Axis variations
    // pub cvar: Option<Table_cvar>, // CVT variations (TrueType outlines only)
    pub fvar: Option<Table_fvar>, // Font variations
    // pub gvar: Option<Table_gvar>, // Glyph variations (TrueType outlines only)
    // pub HVAR: Option<Table_HVAR>, // Horizontal metrics variations
    // pub MVAR: Option<Table_MVAR>, // Metrics variations
    // pub STAT: Option<Table_STAT>, // Style attributes
    // pub VVAR: Option<Table_VVAR>, // Vertical metrics variations
}

// impl fmt::Debug for Font {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         write!(f, "format: {:?}, flavor: {:?}", self.format, self.flavor)
//     }
// }

impl Font {
    #[allow(unused_variables)]
    fn load_sfnt(buffer: &mut Buffer) -> Self {
        let signature = buffer.get::<u32>();
        let num_tables = buffer.get::<u16>();
        let search_range = buffer.get::<u16>();
        let entry_selector = buffer.get::<u16>();
        let range_shift = buffer.get::<u16>();

        // Table Record entries
        let mut table_records = HashMap::new();
        for _ in 0..num_tables {
            table_records.insert(buffer.get::<Tag>(), buffer.get::<TableRecord>());
        }
        Self {
            format: Format::SFNT,
            flavor: Self::_get_flavor(signature),
            table_records,
            head: None,
            hhea: None,
            maxp: None,
            hmtx: None,
            cmap: None,
            name: None,
            OS_2: None,
            post: None,
            avar: None,
            fvar: None,
        }
    }

    #[allow(unused_variables)]
    fn load_woff(buffer: &mut Buffer) -> Self {
        let signature = buffer.get::<u32>();
        let flavor = buffer.get::<u32>();
        let length = buffer.get::<u32>();
        let num_tables = buffer.get::<u16>();
        buffer.skip::<u16>(1);
        let total_sfnt_size = buffer.get::<u32>();
        let major_version = buffer.get::<u16>();
        let minor_version = buffer.get::<u16>();
        let meta_offset = buffer.get::<u32>();
        let meta_length = buffer.get::<u32>();
        let meta_orig_length = buffer.get::<u32>();
        let priv_offset = buffer.get::<u32>();
        let priv_length = buffer.get::<u32>();

        // Table Record entries
        let mut table_records = HashMap::new();
        for _ in 0..num_tables {
            let tag = buffer.get::<Tag>();
            let offset = buffer.get::<u32>();
            let comp_length = buffer.get::<u32>();
            let orig_length = buffer.get::<u32>();
            let orig_checksum = buffer.get::<u32>();
            table_records.insert(
                tag,
                TableRecord {
                    checksum: orig_checksum,
                    offset,
                    length: orig_length,
                    woff_comp_length: comp_length,
                },
            );
        }

        Self {
            format: Format::WOFF,
            flavor: Self::_get_flavor(flavor),
            table_records,
            head: None,
            hhea: None,
            maxp: None,
            hmtx: None,
            cmap: None,
            name: None,
            OS_2: None,
            post: None,
            avar: None,
            fvar: None,
        }
    }

    #[allow(unused_variables)]
    fn load_woff2(buffer: &mut Buffer) -> Self {
        let signature = buffer.get::<u32>();
        let flavor = buffer.get::<u32>();
        let length = buffer.get::<u32>();
        let num_tables = buffer.get::<u16>();
        buffer.skip::<u16>(1);
        let total_sfnt_size = buffer.get::<u32>();
        let total_compressed_size = buffer.get::<u32>();
        let major_version = buffer.get::<u16>();
        let minor_version = buffer.get::<u16>();
        let meta_offset = buffer.get::<u32>();
        let meta_length = buffer.get::<u32>();
        let meta_orig_length = buffer.get::<u32>();
        let priv_offset = buffer.get::<u32>();
        let priv_length = buffer.get::<u32>();

        // TODO: Table Record entries
        let table_records = HashMap::new();

        Self {
            format: Format::WOFF2,
            flavor: Self::_get_flavor(flavor),
            table_records,
            head: None,
            hhea: None,
            maxp: None,
            hmtx: None,
            cmap: None,
            name: None,
            OS_2: None,
            post: None,
            avar: None,
            fvar: None,
        }
    }

    fn _get_flavor(flavor: u32) -> Flavor {
        match flavor {
            SIGNATURE_OTF => Flavor::CFF,
            SIGNATURE_TTF | SIGNATURE_TTF_TRUE | SIGNATURE_TTF_TYP1 => Flavor::TTF,
            // TODO: invalid signature.
            _ => Flavor::CFF,
        }
    }
}

macro_rules! _parse_sfnt {
    ($self:ident, $buffer:ident, $tag:expr, $f:ident) => {
        $buffer.offset = $self.get_table_offset($tag);
        $self.$f($buffer);
    };
}

macro_rules! _parse_woff {
    ($self:ident, $buffer:ident, $tag:expr, $f:ident) => {
        $buffer.offset = $self.get_table_offset($tag);
        let comp_length = $self.get_table_comp_len($tag);
        $self.$f(&mut $buffer.decompress(comp_length));
    };
}

impl Font {
    fn parse_sfnt(&mut self, buffer: &mut Buffer) {
        _parse_sfnt!(self, buffer, "hhea", parse_hhea);
        _parse_sfnt!(self, buffer, "maxp", parse_maxp);
        _parse_sfnt!(self, buffer, "hmtx", parse_hmtx);
        _parse_sfnt!(self, buffer, "cmap", parse_cmap);
        _parse_sfnt!(self, buffer, "name", parse_name);
        _parse_sfnt!(self, buffer, "OS/2", parse_OS_2);
        _parse_sfnt!(self, buffer, "post", parse_post);
        _parse_sfnt!(self, buffer, "fvar", parse_fvar);
    }

    fn parse_woff(&mut self, buffer: &mut Buffer) {
        _parse_woff!(self, buffer, "hhea", parse_hhea);
        _parse_woff!(self, buffer, "maxp", parse_maxp);
        // FIXME: index out of range for slice
        // _parse_woff!(self, buffer, "hmtx", parse_hmtx);
        _parse_woff!(self, buffer, "cmap", parse_cmap);
        _parse_woff!(self, buffer, "name", parse_name);
        // _parse_woff!(self, buffer, "OS/2", parse_OS_2);
        _parse_woff!(self, buffer, "post", parse_post);
        _parse_woff!(self, buffer, "fvar", parse_fvar);
    }

    #[allow(unused_variables)]
    fn parse_woff2(&mut self, buffer: &mut Buffer) {
        unimplemented!()
    }

    fn parse_table_sfnt(&mut self, tag_str: &str, buffer: &mut Buffer) {
        let tag = Tag::new(tag_str);
        match self.table_records.get(&tag) {
            Some(record) => {
                buffer.offset = record.offset as usize;
                self._parse_table(tag_str, buffer);
            },
            None => (),
        }
    }

    fn parse_table_woff(&mut self, tag_str: &str, buffer: &mut Buffer) {
        buffer.offset = self.get_table_offset(tag_str);
        let comp_length = self.get_table_comp_len(tag_str);
        self._parse_table(tag_str, &mut buffer.decompress(comp_length));
    }

    #[allow(unused_variables)]
    fn parse_table_woff2(&mut self, tag_str: &str, buffer: &mut Buffer) {
        unimplemented!()
    }

    fn _parse_table(&mut self, tag_str: &str, buffer: &mut Buffer) {
        match tag_str {
            "head" => self.parse_head(buffer),
            "hhea" => self.parse_hhea(buffer),
            "maxp" => self.parse_maxp(buffer),
            "hmtx" => self.parse_hmtx(buffer),
            "cmap" => self.parse_cmap(buffer),
            "name" => self.parse_name(buffer),
            "OS/2" => self.parse_OS_2(buffer),
            "post" => self.parse_post(buffer),
            "avar" => self.parse_avar(buffer),
            "fvar" => self.parse_fvar(buffer),
            _ => (),
        };
    }

    // TODO: consider Option<>

    fn _get_table_record(&self, tag_str: &str) -> &TableRecord {
        let tag = Tag::new(tag_str);
        self.table_records.get(&tag).unwrap()
    }

    pub fn get_table_len(&self, tag_str: &str) -> usize {
        self._get_table_record(tag_str).length as usize
    }

    pub fn get_table_offset(&self, tag_str: &str) -> usize {
        self._get_table_record(tag_str).offset as usize
    }

    pub fn get_table_comp_len(&self, tag_str: &str) -> usize {
        self._get_table_record(tag_str).woff_comp_length as usize
    }
}

#[derive(Debug)]
enum Format {
    SFNT,
    WOFF,
    WOFF2,
}

#[derive(Debug)]
enum Flavor {
    TTF,
    CFF,
}

#[derive(Debug)]
struct TableRecord {
    checksum: u32,
    offset: u32,
    length: u32,
    woff_comp_length: u32,
}

impl ReadBuffer for TableRecord {
    fn read(buffer: &mut Buffer) -> Self {
        Self {
            checksum: buffer.get::<u32>(),
            offset: buffer.get::<u32>(),
            length: buffer.get::<u32>(),
            woff_comp_length: 0,
        }
    }
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
