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
    num_sizes: u32,
    bitmap_sizes: Vec<BitmapSize>,
    // index_sub_table_arrays: Vec<IndexSubTableArray>,
    index_sub_tables: Vec<IndexSubTable>,
}

impl Font {
    #[allow(non_snake_case)]
    pub fn parse_EBLC(&mut self, buffer: &mut Buffer) {
        let start_offset = buffer.offset;
        let _version = buffer.get_version::<u16>();
        let num_sizes = buffer.get();
        let bitmap_sizes:Vec<BitmapSize> = buffer.get_vec(num_sizes as usize);
        let index_sub_tables = bitmap_sizes.iter().map(|i| {
            buffer.offset = start_offset + i.index_sub_table_array_offset as usize;
            buffer.get()
        }).collect();
        self.EBLC = Some(Table_EBLC {
            _version,
            num_sizes,
            bitmap_sizes,
            index_sub_tables,
        });
    }
}

#[derive(Debug)]
struct BitmapSize {
    index_sub_table_array_offset: u32,
    index_tables_size: u32,
    number_of_index_sub_tables: u32,
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
            index_sub_table_array_offset: buffer.get(),
            index_tables_size: buffer.get(),
            number_of_index_sub_tables: buffer.get(),
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
    // IndexSubTableArray
    first_glyph_index: u16,
    last_glyph_index: u16,
    additional_offset_to_index_sub_table: u32,
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

impl ReadBuffer for IndexSubTable {
    fn read(buffer: &mut Buffer) -> Self {
        let start_offset = buffer.offset;
        let first_glyph_index = buffer.get();
        let last_glyph_index = buffer.get();
        let additional_offset_to_index_sub_table = buffer.get();
        buffer.offset = start_offset + additional_offset_to_index_sub_table as usize;
        let index_format = buffer.get();
        let image_format = buffer.get();
        let image_data_offset = buffer.get();
        let mut sbit_offsets = None;
        let mut image_size = None;
        let mut big_metrics = None;
        let mut num_glyphs = None;
        let mut glyph_array = None;
        let mut glyph_id_array = None;
        let sbit_offsets_size = (last_glyph_index - first_glyph_index + 1) as usize;
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
            first_glyph_index,
            last_glyph_index,
            additional_offset_to_index_sub_table,
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
struct BigGlyphMetrics {
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
struct SmallGlyphMetrics {
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
