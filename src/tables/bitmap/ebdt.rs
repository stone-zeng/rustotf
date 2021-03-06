use std::mem::size_of;

use crate::font::Font;
use crate::tables::bitmap::eblc::{BigGlyphMetrics, SmallGlyphMetrics};
use crate::util::Buffer;

/// ## `EBDT` &mdash; Embedded Bitmap Data Table
///
/// Specification: <https://docs.microsoft.com/en-us/typography/opentype/spec/ebdt>.
///
/// The `EBDT` table is used to embed monochrome or grayscale bitmap glyph
/// data. It is used together with the `EBLC` table, which provides embedded
/// bitmap locators, and the `EBSC` table, which provides embedded bitmap
/// scaling information.

#[allow(non_camel_case_types)]
#[derive(Debug)]
pub struct Table_EBDT {
    version: String,
    pub bitmap_data: Vec<Vec<BitmapData>>,
}

impl Font {
    #[allow(non_snake_case)]
    pub fn parse_EBDT(&mut self, buffer: &mut Buffer) {
        let ebdt_start = buffer.offset();
        let version = buffer.get_version::<u16>();
        let strikes = &self.EBLC.as_ref().unwrap().strikes;
        let bitmap_data = strikes
            .iter()
            .map(|strike| {
                let mut strike_bitmap_data = Vec::new();
                for index_sub_table in &strike.index_sub_tables {
                    buffer.set_offset_from(ebdt_start, index_sub_table.image_data_offset);
                    match index_sub_table.image_format {
                        1 | 2 => {
                            // TODO: only for index format 1 or 3
                            let sbit = index_sub_table.sbit_offsets.as_ref().unwrap();
                            (0..sbit.len() - 1).for_each(|i| {
                                let image_data_size = (sbit[i + 1] - sbit[i]) as usize
                                    - size_of::<SmallGlyphMetrics>();
                                strike_bitmap_data.push(BitmapData {
                                    small_metrics: Some(buffer.get()),
                                    image_data: Some(buffer.get_vec(image_data_size)),
                                    ..Default::default()
                                });
                            })
                        }
                        5 => {
                            // TODO: only for index format 2
                            let image_size = index_sub_table.image_size.unwrap();
                            let len = index_sub_table.last_glyph_index
                                - index_sub_table.first_glyph_index
                                + 1;
                            (0..len).for_each(|_| {
                                strike_bitmap_data.push(BitmapData {
                                    image_data: Some(buffer.get_vec(image_size)),
                                    ..Default::default()
                                });
                            })
                        }
                        6 | 7 | 8 | 9 => unimplemented!(),
                        _ => unreachable!(),
                    }
                }
                strike_bitmap_data
            })
            .collect();
        self.EBDT = Some(Table_EBDT {
            version,
            bitmap_data,
        });
    }
}

#[derive(Debug, Default)]
pub struct BitmapData {
    pub small_metrics: Option<SmallGlyphMetrics>,
    pub big_metrics: Option<BigGlyphMetrics>,
    pub image_data: Option<Vec<u8>>,
    pub pad: Option<u8>,
    pub num_components: Option<u16>,
    pub components: Option<Vec<EbdtComponent>>,
}

#[derive(Debug)]
pub struct EbdtComponent {
    glyph_id: u16,
    x_offset: i8,
    y_offset: i8,
}
