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
    pub strikes: Vec<Strikes>,
}

impl Font {
    pub fn parse_sbix(&mut self, buffer: &mut Buffer) {
        let sbix_start_offset = buffer.offset();
        let num_glyphs = self.maxp.as_ref().unwrap().num_glyphs as usize;
        let _version: u16 = buffer.get();
        let _flags: u16 = buffer.get();
        let num_strikes: u32 = buffer.get();
        let strike_offsets: Vec<u32> = buffer.get_vec(num_strikes);
        let strikes = strike_offsets
            .iter()
            .map(|&strike_offset| {
                buffer.set_offset_from(sbix_start_offset, strike_offset);
                Strikes::read(buffer, num_glyphs)
            })
            .collect();
        self.sbix = Some(Table_sbix { strikes });
    }
}

#[derive(Debug)]
pub struct Strikes {
    pub ppem: u16,
    pub ppi: u16,
    pub glyph_data: Vec<GlyphData>,
}

impl Strikes {
    fn read(buffer: &mut Buffer, num_glyphs: usize) -> Self {
        let start_offset = buffer.offset();
        let ppem = buffer.get();
        let ppi = buffer.get();
        let glyph_data_offsets: Vec<u32> = buffer.get_vec(num_glyphs + 1);
        let glyph_data = (0..num_glyphs)
            .map(|i| {
                buffer.set_offset_from(start_offset, glyph_data_offsets[i]);
                let data_len = glyph_data_offsets[i + 1] - glyph_data_offsets[i];
                GlyphData::read(buffer, data_len as usize)
            })
            .collect();
        Self {
            ppem,
            ppi,
            glyph_data,
        }
    }
}

#[derive(Debug)]
pub struct GlyphData {
    pub origin_offset_x: i16,
    pub origin_offset_y: i16,
    pub graphic_type: Tag,
    pub data: Vec<u8>,
}

impl GlyphData {
    fn read(buffer: &mut Buffer, data_len: usize) -> Self {
        Self {
            origin_offset_x: buffer.get(),
            origin_offset_y: buffer.get(),
            graphic_type: buffer.get(),
            data: match data_len {
                0 => vec![],
                _ => buffer.get_vec(data_len - 8),
            },
        }
    }
}
