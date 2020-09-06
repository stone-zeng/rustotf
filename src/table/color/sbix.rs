use crate::font::Font;
use crate::util::{Buffer, Tag};

/// ## `sbix` &mdash; Standard Bitmap Graphics Table
///
/// Specification: <https://docs.microsoft.com/en-us/typography/opentype/spec/sbix>.
///
/// This table provides access to bitmap data in a standard graphics format, such as
/// PNG, JPEG or TIFF.

#[allow(non_camel_case_types)]
#[derive(Debug)]
pub struct Table_sbix {
    strikes: Vec<Strikes>,
}

impl Font {
    pub fn parse_sbix(&mut self, buffer: &mut Buffer) {
        let sbix_start_offset = buffer.offset;
        let num_glyphs = self.maxp.as_ref().unwrap().num_glyphs as usize;

        let _version = buffer.get::<u16>();
        let _flags = buffer.get::<u16>();
        let num_strikes = buffer.get::<u32>();
        let strike_offsets = buffer.get_vec::<u32>(num_strikes as usize);

        self.sbix = Some(Table_sbix {
            strikes: strike_offsets.iter().map(|&strike_offset| {
                let strike_start_offset = sbix_start_offset + strike_offset as usize;
                buffer.offset = strike_start_offset;
                let ppem = buffer.get();
                let ppi = buffer.get();
                let glyph_data_offsets = buffer.get_vec::<u32>(num_glyphs + 1);
                Strikes {
                    ppem,
                    ppi,
                    glyph_data: (0..num_glyphs).map(|i| {
                        buffer.offset = strike_start_offset + glyph_data_offsets[i] as usize;
                        let data_len = glyph_data_offsets[i + 1] - glyph_data_offsets[i];
                        GlyphData {
                            origin_offset_x: buffer.get(),
                            origin_offset_y: buffer.get(),
                            graphic_type: buffer.get(),
                            data: match data_len {
                                0 => vec![],
                                _ => buffer.get_vec((data_len - 8) as usize),
                            },
                        }
                    }).collect()
                }
            }).collect()
        });
    }
}

#[derive(Debug)]
struct Strikes {
    ppem: u16,
    ppi: u16,
    glyph_data: Vec<GlyphData>,
}

#[derive(Debug)]
struct GlyphData {
    origin_offset_x: i16,
    origin_offset_y: i16,
    graphic_type: Tag,
    data: Vec<u8>,
}
