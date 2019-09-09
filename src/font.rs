use crate::table::{
    cmap::Table_cmap, head::Table_head, hhea::Table_hhea, hmtx::Table_hmtx, maxp::Table_maxp,
    name::Table_name, os_2::Table_OS_2, post::Table_post,
};
use crate::util::{Buffer, ReadBuffer, Tag};

use std::collections::HashMap;
use std::error::Error;
use std::fs;

pub fn read_font(font_file_path: &str) -> Result<(), Box<dyn Error>> {
    println!("{:?}", font_file_path);

    let mut font_container = FontContainer::new(fs::read(font_file_path)?);
    font_container.init();
    font_container.parse();

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
            // 'OTTO' | .. | 'true' | 'typ1'
            0x4F54_544F | 0x0001_0000 | 0x7472_7565 | 0x7479_7031 => self.init_otf(),
            // 'ttcf'
            0x7474_6366 => self.init_ttc(),
            // 'wOFF'
            0x774F_4646 => self.init_woff(),
            // 'wOF2'
            0x774F_4632 => self.init_woff2(),
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

    pub fn parse_table(&mut self, tag: &Tag) {
        for font in &mut self.fonts {
            match font.format {
                Format::SFNT => font.parse_table_sfnt(tag, &mut self.buffer),
                Format::WOFF => font.parse_table_woff(tag, &mut self.buffer),
                Format::WOFF2 => font.parse_table_woff2(tag, &mut self.buffer),
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
}

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
        }
    }

    #[allow(unused_variables)]
    fn load_woff(buffer: &mut Buffer) -> Self {
        let signature = buffer.get::<u32>();
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
        }
    }

    #[allow(unused_variables)]
    fn load_woff2(buffer: &mut Buffer) -> Self {
        let signature = buffer.get::<u32>();
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
        }
    }

    fn _get_flavor(flavor: u32) -> Flavor {
        match flavor {
            // 'OTTO'
            0x4F54_544F => Flavor::CFF,
            // .. | 'true' | 'typ1'
            0x0001_0000 | 0x7472_7565 | 0x7479_7031 => Flavor::TTF,
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
        let comp_len = $self.get_table_comp_len($tag);
        $self.$f(&mut $buffer.decompress(comp_len));
    };
}

impl Font {
    fn parse_sfnt(&mut self, buffer: &mut Buffer) {
        _parse_sfnt!(self, buffer, b"hhea", parse_hhea);
        _parse_sfnt!(self, buffer, b"maxp", parse_maxp);
        _parse_sfnt!(self, buffer, b"hmtx", parse_hmtx);
        _parse_sfnt!(self, buffer, b"cmap", parse_cmap);
        _parse_sfnt!(self, buffer, b"name", parse_name);
        _parse_sfnt!(self, buffer, b"OS/2", parse_OS_2);
        _parse_sfnt!(self, buffer, b"post", parse_post);
    }

    fn parse_woff(&mut self, buffer: &mut Buffer) {
        _parse_woff!(self, buffer, b"hhea", parse_hhea);
        _parse_woff!(self, buffer, b"maxp", parse_maxp);
        // FIXME: index out of range for slice
        // _parse_woff!(self, buffer, b"hmtx", parse_hmtx);
        _parse_woff!(self, buffer, b"cmap", parse_cmap);
        _parse_woff!(self, buffer, b"name", parse_name);
        // _parse_woff!(self, buffer, b"OS/2", parse_OS_2);
        _parse_woff!(self, buffer, b"post", parse_post);
    }

    #[allow(unused_variables)]
    fn parse_woff2(&mut self, buffer: &mut Buffer) {
        unimplemented!()
    }

    fn parse_table_sfnt(&mut self, tag: &Tag, buffer: &mut Buffer) {
        buffer.offset = self.get_table_offset(tag);
        self._parse_table(tag, buffer);
    }

    fn parse_table_woff(&mut self, tag: &Tag, buffer: &mut Buffer) {
        buffer.offset = self.get_table_offset(tag);
        let comp_len = self.get_table_comp_len(tag);
        self._parse_table(tag, &mut buffer.decompress(comp_len));
    }

    #[allow(unused_variables)]
    fn parse_table_woff2(&mut self, tag: &Tag, buffer: &mut Buffer) {
        unimplemented!()
    }

    fn _parse_table(&mut self, tag: &Tag, buffer: &mut Buffer) {
        match tag {
            b"head" => self.parse_head(buffer),
            b"hhea" => self.parse_hhea(buffer),
            b"maxp" => self.parse_maxp(buffer),
            b"hmtx" => self.parse_hmtx(buffer),
            b"cmap" => self.parse_cmap(buffer),
            b"name" => self.parse_name(buffer),
            b"OS/2" => self.parse_OS_2(buffer),
            b"post" => self.parse_post(buffer),
            _ => (),
        };
    }

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
