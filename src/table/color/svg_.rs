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
    _version: u16,
    pub num_entries: u16,
    pub doc_records: Vec<SvgDocumentRecord>,
}

impl Font {
    #[allow(non_snake_case)]
    pub fn parse_SVG_(&mut self, buffer: &mut Buffer) {
        let svg_start_offset = buffer.offset;
        let _version = buffer.get();
        let start_offset = svg_start_offset + buffer.get::<u32>() as usize;
        buffer.offset = start_offset;
        let num_entries = buffer.get();
        let doc_records = (0..num_entries)
            .map(|_| SvgDocumentRecord::read(buffer, start_offset))
            .collect();
        self.SVG_ = Some(Table_SVG_ {
            _version,
            num_entries,
            doc_records,
        })
    }
}

#[derive(Debug)]
pub struct SvgDocumentRecord {
    pub start_glyph_id: u16,
    pub end_glyph_id: u16,
    pub svg_doc: String,
}

impl SvgDocumentRecord {
    fn read(buffer: &mut Buffer, start_offset: usize) -> Self {
        let offset = buffer.offset;
        let start_glyph_id = buffer.get();
        let end_glyph_id = buffer.get();
        let svg_doc_offset: u32 = buffer.get();
        let svg_doc_length: u32 = buffer.get();
        buffer.offset = start_offset + svg_doc_offset as usize;
        let svg_doc = Self::get_svg_doc(buffer, svg_doc_length as usize);
        buffer.offset = offset + 12; // u16 + u16 + u32 + u32
        Self {
            start_glyph_id,
            end_glyph_id,
            svg_doc,
        }
    }

    fn get_svg_doc(buffer: &mut Buffer, len: usize) -> String {
        let utf8 = if len > 3 && Self::check_gzip_header(buffer) {
            let mut orig_buffer = buffer.gz_decompress(len);
            orig_buffer.get_vec(orig_buffer.len())
        } else {
            buffer.get_vec(len)
        };
        String::from_utf8(utf8).unwrap()
    }

    fn check_gzip_header(buffer: &mut Buffer) -> bool {
        let header: Vec<u8> = buffer.get_vec(3);
        buffer.offset -= 3; // 3 * u8
        header == vec![0x1F, 0x8B, 0x08]
    }
}
