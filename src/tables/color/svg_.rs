use crate::font::Font;
use crate::util::Buffer;

/// ## `SVG` &mdash; The SVG (Scalable Vector Graphics) Table
///
/// Specification: <https://docs.microsoft.com/en-us/typography/opentype/spec/svg>.
///
/// This table contains SVG descriptions for some or all of the glyphs in the font.

#[allow(non_camel_case_types)]
#[derive(Debug)]
pub struct Table_SVG_ {
    version: u16,
    pub num_entries: u16,
    pub doc_records: Vec<SvgDocRecord>,
}

impl Font {
    #[allow(non_snake_case)]
    pub fn parse_SVG_(&mut self, buffer: &mut Buffer) {
        let svg_start = buffer.offset();
        let version = buffer.get();
        let svg_doc_list_offset: u32 = buffer.get();
        buffer.set_offset_from(svg_start, svg_doc_list_offset);
        let svg_doc_start = buffer.offset();
        let num_entries = buffer.get();
        let doc_records = (0..num_entries)
            .map(|_| SvgDocRecord::read(buffer, svg_doc_start))
            .collect();
        self.SVG_ = Some(Table_SVG_ {
            version,
            num_entries,
            doc_records,
        })
    }
}

#[derive(Debug)]
pub struct SvgDocRecord {
    pub start_glyph_id: u16,
    pub end_glyph_id: u16,
    pub svg_doc: String,
}

impl SvgDocRecord {
    fn read(buffer: &mut Buffer, start: usize) -> Self {
        let offset = buffer.offset();
        let start_glyph_id = buffer.get();
        let end_glyph_id = buffer.get();
        let svg_doc_offset: u32 = buffer.get();
        let svg_doc_length: u32 = buffer.get();
        buffer.set_offset_from(start, svg_doc_offset);
        let svg_doc = Self::get_svg_doc(buffer, svg_doc_length as usize);
        buffer.set_offset(offset + 12); // u16 + u16 + u32 + u32
        Self {
            start_glyph_id,
            end_glyph_id,
            svg_doc,
        }
    }

    fn get_svg_doc(buffer: &mut Buffer, len: usize) -> String {
        let utf8 = if len > 3 && Self::check_gzip_header(buffer) {
            let mut orig_buffer = buffer.gz_decompress(len).unwrap();
            orig_buffer.get_vec(orig_buffer.len())
        } else {
            buffer.get_vec(len)
        };
        String::from_utf8(utf8).unwrap()
    }

    fn check_gzip_header(buffer: &mut Buffer) -> bool {
        let start = buffer.offset();
        let header: Vec<u8> = buffer.get_vec(GZIP_HEADER.len());
        buffer.set_offset(start);
        header == GZIP_HEADER
    }
}

const GZIP_HEADER: &[u8] = &[0x1F, 0x8B, 0x08];
