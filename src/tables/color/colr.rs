use crate::font::Font;
use crate::util::{Buffer, ReadBuffer};
use read_buffer_derive::ReadBuffer;

/// ## `COLR` &mdash; Color Table
///
/// Specification: <https://docs.microsoft.com/en-us/typography/opentype/spec/colr>.
///
/// The `COLR` table adds support for multi-colored glyphs in a manner that is compatible
/// with existing text engines and easy to support with current OpenType font files.

#[allow(non_camel_case_types)]
#[derive(Debug)]
pub struct Table_COLR {
    _version: u16,
    pub color_glyphs: Vec<ColorGlyph>,
}

impl Font {
    #[allow(non_snake_case)]
    pub fn parse_COLR(&mut self, buffer: &mut Buffer) {
        let colr_start_offset = buffer.offset();
        let _version = buffer.get();
        let num_base_glyph_records: u16 = buffer.get();
        let base_glyph_records_offset: u32 = buffer.get();
        let layer_records_offset: u32 = buffer.get();
        let num_layer_records: u16 = buffer.get();

        buffer.set_offset_from(colr_start_offset, base_glyph_records_offset);
        let base_glyph_records: Vec<BaseGlyphRecord> =
            buffer.get_vec(num_base_glyph_records);

        buffer.set_offset_from(colr_start_offset, layer_records_offset);
        let layer_records: Vec<Layer> = buffer.get_vec(num_layer_records);

        let color_glyphs = base_glyph_records
            .iter()
            .map(|rec| {
                let layers = (0..rec.num_layers)
                    .map(|i| layer_records[(rec.first_layer_index + i) as usize])
                    .collect();
                ColorGlyph {
                    glyph_id: rec.glyph_id,
                    layers,
                }
            })
            .collect();

        self.COLR = Some(Table_COLR {
            _version,
            color_glyphs,
        });
    }
}

#[derive(Debug)]
pub struct ColorGlyph {
    pub glyph_id: u16,
    pub layers: Vec<Layer>,
}

#[derive(Debug, ReadBuffer)]
struct BaseGlyphRecord {
    glyph_id: u16,
    first_layer_index: u16,
    num_layers: u16,
}

#[derive(Debug, ReadBuffer, Clone, Copy)]
pub struct Layer {
    pub glyph_id: u16,
    pub palette_index: u16,
}
