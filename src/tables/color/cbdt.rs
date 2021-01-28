use crate::font::Font;
use crate::tables::bitmap::ebdt::BitmapData;
use crate::util::Buffer;

/// ## `CBDT` &mdash; Color Bitmap Data Table
///
/// Specification: <https://docs.microsoft.com/en-us/typography/opentype/spec/cbdt>.
///
/// The `CBDT` table is used to embed color bitmap glyph data. It is used together with the
/// `CBLC` table, which provides embedded bitmap locators. The formats of these two tables
/// are backward compatible with the `EBDT` and `EBLC` tables used for embedded monochrome
/// and grayscale bitmaps.

#[allow(non_camel_case_types)]
#[derive(Debug)]
pub struct Table_CBDT {
    version: String,
    pub bitmap_data: Vec<Vec<BitmapData>>,
}

impl Font {
    #[allow(non_snake_case)]
    pub fn parse_CBDT(&mut self, buffer: &mut Buffer) {
        let cbdt_start = buffer.offset();
        let version = buffer.get_version::<u16>();
        let strikes = &self.CBLC.as_ref().unwrap().strikes;
        let bitmap_data = strikes
            .iter()
            .map(|strike| {
                let mut strike_bitmap_data = Vec::new();
                for index_sub_table in &strike.index_sub_tables {
                    buffer.set_offset_from(cbdt_start, index_sub_table.image_data_offset);
                    match index_sub_table.image_format {
                        17 => {
                            let len = index_sub_table.sbit_offsets.as_ref().unwrap().len() - 1;
                            (0..len).for_each(|_| {
                                let small_metrics = Some(buffer.get());
                                let data_len: u32 = buffer.get();
                                let image_data = Some(buffer.get_vec(data_len));
                                strike_bitmap_data.push(BitmapData {
                                    small_metrics,
                                    image_data,
                                    ..Default::default()
                                })
                            })
                        }
                        _ => unimplemented!(),
                    }
                }
                strike_bitmap_data
            })
            .collect();
        self.CBDT = Some(Table_CBDT {
            version,
            bitmap_data,
        });
    }
}
