use crate::font::Font;
use crate::util::{Buffer, ReadBuffer};

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
    strikes: Vec<Strike>,
}

impl Font {
    #[allow(non_snake_case)]
    pub fn parse_EBLC(&mut self, buffer: &mut Buffer) {
        let eblc_start_offset = buffer.offset;
        let _version = buffer.get_version::<u16>();
        let _num_strikes = buffer.get();
        self.EBLC = Some(Table_EBLC {
            _version,
            _num_strikes,
            strikes: Strike::read_vec(buffer, _num_strikes as usize, eblc_start_offset),
        })
    }
}

#[derive(Debug)]
struct Strike {
    bitmap_size: BitmapSize,
    index_sub_tables: Vec<IndexSubTable>,
}

impl Strike {
    fn read_vec(buffer: &mut Buffer, num: usize, eblc_start_offset: usize) -> Vec<Self> {
        let bitmap_size_vec: Vec<BitmapSize> = buffer.get_vec(num);
        let mut strikes = Vec::new();
        for bitmap_size in bitmap_size_vec {
            let index_sub_table_start_offset =
                eblc_start_offset + bitmap_size._index_sub_table_offset as usize;
            buffer.offset = index_sub_table_start_offset;
            let index_sub_table_arrays: Vec<IndexSubTableArray> =
                buffer.get_vec(bitmap_size._num_index_sub_tables as usize);
            let index_sub_tables = index_sub_table_arrays.iter().map(|i| {
                buffer.offset = index_sub_table_start_offset + i.additional_offset as usize;
                IndexSubTable::read(buffer, i)
            }).collect();
            strikes.push(Self {
                bitmap_size,
                index_sub_tables,
            });
        }
        strikes
    }
}

#[derive(Debug)]
struct BitmapSize {
    _index_sub_table_offset: u32,
    _index_sub_tables_size: u32,
    _num_index_sub_tables: u32,
    color_ref: u32,
    hori: SbitLineMetrics,
    vert: SbitLineMetrics,
    start_glyph_index: u16,
    end_glyph_index: u16,
    ppem_x: u8,
    ppem_y: u8,
    bit_depth: u8,
    flags: i8,
}

impl ReadBuffer for BitmapSize {
    fn read(buffer: &mut Buffer) -> Self {
        Self {
            _index_sub_table_offset: buffer.get(),
            _index_sub_tables_size: buffer.get(),
            _num_index_sub_tables: buffer.get(),
            color_ref: buffer.get(),
            hori: buffer.get(),
            vert: buffer.get(),
            start_glyph_index: buffer.get(),
            end_glyph_index: buffer.get(),
            ppem_x: buffer.get(),
            ppem_y: buffer.get(),
            bit_depth: buffer.get(),
            flags: buffer.get(),
        }
    }
}

#[derive(Debug)]
struct SbitLineMetrics {
    ascender: i8,
    descender: i8,
    width_max: u8,
    caret_slope_numerator: i8,
    caret_slope_denominator: i8,
    caret_offset: i8,
    min_origin_sb: i8,
    min_advance_sb: i8,
    max_before_bl: i8,
    min_after_bl: i8,
    pad1: i8,
    pad2: i8,
}

impl ReadBuffer for SbitLineMetrics {
    fn read(buffer: &mut Buffer) -> Self {
        Self {
            ascender: buffer.get(),
            descender: buffer.get(),
            width_max: buffer.get(),
            caret_slope_numerator: buffer.get(),
            caret_slope_denominator: buffer.get(),
            caret_offset: buffer.get(),
            min_origin_sb: buffer.get(),
            min_advance_sb: buffer.get(),
            max_before_bl: buffer.get(),
            min_after_bl: buffer.get(),
            pad1: buffer.get(),
            pad2: buffer.get(),
        }
    }
}

#[derive(Debug)]
struct IndexSubTable {
    first_glyph_index: u16,
    last_glyph_index: u16,
    // Header
    index_format: u16,
    image_format: u16,
    image_data_offset: u32,
    // Format 1, 3
    sbit_offsets: Option<Vec<u32>>,
    // Format 2, 5
    image_size: Option<u32>,
    big_metrics: Option<BigGlyphMetrics>,
    // Format 4, 5
    num_glyphs: Option<u32>,
    glyph_array: Option<Vec<GlyphIdOffsetPair>>,
    // Format 5
    glyph_id_array: Option<Vec<u16>>,
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
        let sbit_offsets_size = (array.last_glyph_index - array.first_glyph_index + 1) as usize;
        match index_format {
            1 => {
                sbit_offsets = Some(buffer.get_vec(sbit_offsets_size));
            }
            2 => {
                image_size = Some(buffer.get());
                big_metrics = Some(buffer.get());
            }
            3 => {
                let sbit_offsets_u16: Vec<u16> = buffer.get_vec(sbit_offsets_size);
                sbit_offsets = Some(sbit_offsets_u16.iter().map(|&i| i as u32).collect());
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

#[derive(Debug)]
struct IndexSubTableArray {
    first_glyph_index: u16,
    last_glyph_index: u16,
    additional_offset: u32,
}

impl ReadBuffer for IndexSubTableArray {
    fn read(buffer: &mut Buffer) -> Self {
        Self {
            first_glyph_index: buffer.get(),
            last_glyph_index: buffer.get(),
            additional_offset: buffer.get(),
        }
    }
}

#[derive(Debug)]
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

impl ReadBuffer for BigGlyphMetrics {
    fn read(buffer: &mut Buffer) -> Self {
        Self {
            height: buffer.get(),
            width: buffer.get(),
            hori_bearing_x: buffer.get(),
            hori_bearing_y: buffer.get(),
            hori_advance: buffer.get(),
            vert_bearing_x: buffer.get(),
            vert_bearing_y: buffer.get(),
            vert_advance: buffer.get(),
        }
    }
}

#[derive(Debug)]
pub struct SmallGlyphMetrics {
    height: u8,
    width: u8,
    bearing_x: i8,
    bearing_y: i8,
    advance: u8,
}

impl ReadBuffer for SmallGlyphMetrics {
    fn read(buffer: &mut Buffer) -> Self {
        Self {
            height: buffer.get(),
            width: buffer.get(),
            bearing_x: buffer.get(),
            bearing_y: buffer.get(),
            advance: buffer.get(),
        }
    }
}

#[derive(Debug)]
struct GlyphIdOffsetPair {
    glyph_id: u16,
    sbit_offset: u16,
}

impl ReadBuffer for GlyphIdOffsetPair {
    fn read(buffer: &mut Buffer) -> Self {
        Self {
            glyph_id: buffer.get(),
            sbit_offset: buffer.get(),
        }
    }
}
