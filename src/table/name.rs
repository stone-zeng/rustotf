use crate::font::{Font, TableRecord};
use crate::util::Buffer;

/// ## `name` &mdash; Naming Table
///
/// <https://docs.microsoft.com/en-us/typography/opentype/spec/name>
///
/// The naming table allows multilingual strings to be associated with the
/// OpenType font. These strings can represent copyright notices, font names,
/// family names, style names, and so on. To keep this table short, the font
/// manufacturer may wish to make a limited set of entries in some small set of
/// languages; later, the font can be "localized" and the strings translated or
/// added. Other parts of the OpenType font that require these strings can refer
/// to them using a language-independent name ID. In addition to language
/// variants, the table also allows for platform-specific character-encoding
/// variants. Clients that need a particular string can look it up by its
/// platform ID, encoding ID, language ID and name ID. Note that different
/// platforms may have different requirements for the encoding of strings.
///
/// Many newer platforms can use strings intended for different platforms if a
/// font does not include strings for that platform. Some applications might
/// display incorrect strings, however, if strings for the current platform are
/// not included.

#[allow(non_camel_case_types)]
#[derive(Debug)]
pub struct Table_name {}

impl Font {
    pub fn parse_name(&mut self, buffer: &mut Buffer, record: &TableRecord) {
        buffer.offset = record.offset as usize;
    }
}
