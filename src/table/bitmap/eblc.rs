use crate::font::Font;
use crate::util::{Buffer, ReadBuffer};
use read_buffer_derive::ReadBuffer;

/// ## `EBLC` &mdash; Embedded Bitmap Location Table
///
/// Specification: <https://docs.microsoft.com/en-us/typography/opentype/spec/eblc>.
///
/// The `EBLC` provides embedded bitmap locators. It is used together with the
/// `EBDT` table, which provides embedded, monochrome or grayscale bitmap
/// glyph data, and the `EBSC` table, which provided embedded bitmap scaling
/// information.

#[allow(non_camel_case_types)]
#[derive(Debug)]
pub struct Table_EBLC {
    _version: String,
    _num_strikes: u32,
    pub strikes: Vec<Strike>,
}

impl Font {
    #[allow(non_snake_case)]
    pub fn parse_EBLC(&mut self, buffer: &mut Buffer) {
        let eblc_start_offset = buffer.offset;
        let _version = buffer.get_version::<u16>();
        let _num_strikes = buffer.get();
        let strikes = Strike::read_vec(buffer, _num_strikes as usize, eblc_start_offset);
        self.EBLC = Some(Table_EBLC {
            _version,
            _num_strikes,
            strikes,
        })
    }
}

#[derive(Debug)]
pub struct Strike {
    pub bitmap_size: BitmapSize,
    pub index_sub_tables: Vec<IndexSubTable>,
}

impl Strike {
    pub fn read_vec(buffer: &mut Buffer, num: usize, start_offset: usize) -> Vec<Self> {
        let bitmap_size_vec: Vec<BitmapSize> = buffer.get_vec(num);
        let mut strikes = Vec::new();
        for bitmap_size in bitmap_size_vec {
            let index_sub_table_start_offset =
                start_offset + bitmap_size._index_sub_table_offset as usize;
            buffer.offset = index_sub_table_start_offset;
            let index_sub_table_arrays: Vec<IndexSubTableArray> =
                buffer.get_vec(bitmap_size._num_index_sub_tables as usize);
            let index_sub_tables = index_sub_table_arrays
                .iter()
                .map(|i| {
                    buffer.offset = index_sub_table_start_offset + i.additional_offset as usize;
                    IndexSubTable::read(buffer, i)
                })
                .collect();
            strikes.push(Self {
                bitmap_size,
                index_sub_tables,
            });
        }
        strikes
    }
}

#[derive(Debug, ReadBuffer)]
pub struct BitmapSize {
    _index_sub_table_offset: u32,
    _index_sub_tables_size: u32,
    _num_index_sub_tables: u32,
    pub color_ref: u32,
    pub hori: SbitLineMetrics,
    pub vert: SbitLineMetrics,
    pub start_glyph_index: u16,
    pub end_glyph_index: u16,
    pub ppem_x: u8,
    pub ppem_y: u8,
    pub bit_depth: u8,
    pub flags: i8,
}

#[derive(Debug, ReadBuffer)]
pub struct SbitLineMetrics {
    pub ascender: i8,
    pub descender: i8,
    pub width_max: u8,
    pub caret_slope_numerator: i8,
    pub caret_slope_denominator: i8,
    pub caret_offset: i8,
    pub min_origin_sb: i8,
    pub min_advance_sb: i8,
    pub max_before_bl: i8,
    pub min_after_bl: i8,
    pub pad1: i8,
    pub pad2: i8,
}

#[derive(Debug)]
pub struct IndexSubTable {
    pub first_glyph_index: u16,
    pub last_glyph_index: u16,
    // Header
    pub index_format: u16,
    pub image_format: u16,
    pub image_data_offset: u32,
    // Format 1, 3
    pub sbit_offsets: Option<Vec<u32>>,
    // Format 2, 5
    pub image_size: Option<u32>,
    pub big_metrics: Option<BigGlyphMetrics>,
    // Format 4, 5
    pub num_glyphs: Option<u32>,
    pub glyph_array: Option<Vec<GlyphIdOffsetPair>>,
    // Format 5
    pub glyph_id_array: Option<Vec<u16>>,
}

impl IndexSubTable {
    fn read(buffer: &mut Buffer, array: &IndexSubTableArray) -> Self {
        let index_format = buffer.get();
        let image_format = buffer.get();
        let image_data_offset = buffer.get();
        let mut sbit_offsets = None;
        let mut image_size = None;
        let mut big_metrics = None;
        let mut num_glyphs = None;
        let mut glyph_array = None;
        let mut glyph_id_array = None;
        let sbit_offsets_size = (array.last_glyph_index - array.first_glyph_index + 2) as usize;
        match index_format {
            1 => {
                sbit_offsets = Some(buffer.get_vec(sbit_offsets_size));
            }
            2 => {
                image_size = Some(buffer.get());
                big_metrics = Some(buffer.get());
            }
            3 => {
                sbit_offsets = Some(
                    (0..sbit_offsets_size)
                        .map(|_| buffer.get::<u16>() as u32)
                        .collect(),
                );
            }
            4 => {
                num_glyphs = Some(buffer.get());
                glyph_array = Some(buffer.get_vec(num_glyphs.unwrap() as usize + 1));
            }
            5 => {
                image_size = Some(buffer.get());
                big_metrics = Some(buffer.get());
                num_glyphs = Some(buffer.get());
                glyph_id_array = Some(buffer.get_vec(num_glyphs.unwrap() as usize));
            }
            _ => unreachable!(),
        }
        Self {
            first_glyph_index: array.first_glyph_index,
            last_glyph_index: array.last_glyph_index,
            index_format,
            image_format,
            image_data_offset,
            sbit_offsets,
            image_size,
            big_metrics,
            num_glyphs,
            glyph_array,
            glyph_id_array,
        }
    }
}

#[derive(Debug, ReadBuffer)]
struct IndexSubTableArray {
    first_glyph_index: u16,
    last_glyph_index: u16,
    additional_offset: u32,
}

#[derive(Debug, ReadBuffer)]
pub struct BigGlyphMetrics {
    height: u8,
    width: u8,
    hori_bearing_x: i8,
    hori_bearing_y: i8,
    hori_advance: u8,
    vert_bearing_x: i8,
    vert_bearing_y: i8,
    vert_advance: u8,
}

#[derive(Debug, ReadBuffer)]
pub struct SmallGlyphMetrics {
    height: u8,
    width: u8,
    bearing_x: i8,
    bearing_y: i8,
    advance: u8,
}

#[derive(Debug, ReadBuffer)]
pub struct GlyphIdOffsetPair {
    pub glyph_id: u16,
    pub sbit_offset: u16,
}
