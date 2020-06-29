use crate::font::Font;
use crate::util::Buffer;

/// ## `CFF` &mdash; Compact Font Format table
///
/// Specification: <https://docs.microsoft.com/en-us/typography/opentype/spec/cff>.
///
/// This table contains a Compact Font Format font representation (also known as a PostScript
/// Type 1, or CIDFont) and is structured according to
/// [*Adobe Technical Note #5176: The Compact Font Format Specification*](https://wwwimages2.adobe.com/content/dam/acom/en/devnet/font/pdfs/5176.CFF.pdf)
/// and [*Adobe Technical Note #5177: Type 2 Charstring Format*](https://wwwimages2.adobe.com/content/dam/acom/en/devnet/font/pdfs/5177.Type2.pdf).

#[allow(non_camel_case_types)]
#[derive(Debug)]
pub struct Table_CFF_ {
    _version: String,
    header_size: u8,
    offset_size: u8,
}

impl Font {
    #[allow(non_snake_case)]
    pub fn parse_CFF_(&mut self, buffer: &mut Buffer) {
        self.CFF_ = Some(Table_CFF_ {
            _version: buffer.get_version::<u8>(),
            header_size: buffer.get::<u8>(),
            offset_size: buffer.get::<u8>(),
        });
    }
}
