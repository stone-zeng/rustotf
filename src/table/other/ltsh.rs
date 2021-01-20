use crate::font::Font;
use crate::util::Buffer;

/// ## `LTSH` &mdash; Linear Threshold
///
/// Specification: <https://docs.microsoft.com/en-us/typography/opentype/spec/ltsh>.
///
/// The `LTSH` table relates to OpenType fonts containing TrueType outlines. There are
/// noticeable improvements to fonts on the screen when instructions are carefully applied
/// to the sidebearings. The gain in readability is offset by the necessity for the OS to
/// grid fit the glyphs in order to find the actual advance width for the glyphs (since
/// instructions may be moving the sidebearing points). The TrueType outline format already
/// has two mechanisms to side step the speed issues: the `hdmx` table, where precomputed
/// advance widths may be saved for selected ppem sizes, and the `VDMX` table, where
/// precomputed vertical advance widths may be saved for selected ppem sizes. The `LTSH`
/// table (Linear ThreSHold) is a second, complementary method.

#[allow(non_camel_case_types)]
#[derive(Debug)]
pub struct Table_LTSH {
    version: u16,
    pub num_glyphs: u16,
    pub y_pels: Vec<u8>,
}

impl Font {
    #[allow(non_snake_case)]
    pub fn parse_LTSH(&mut self, buffer: &mut Buffer) {
        let version = buffer.get();
        let num_glyphs = buffer.get();
        let y_pels = buffer.get_vec(num_glyphs as usize);
        self.LTSH = Some(Table_LTSH {
            version,
            num_glyphs,
            y_pels,
        });
    }
}
