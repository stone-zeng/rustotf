use crate::font::{Font, TableRecord};
use crate::util::{Buffer, Offset16, Read};

use encoding_rs;

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
pub struct Table_name {
    _format: u16,
    _count: u16,
    _string_offset: Offset16,
    _names: Vec<Name>,
    _lang_tag_count: Option<u16>,
    _lang_tags: Option<Vec<LangTag>>,
}

impl Font {
    pub fn parse_name(&mut self, buffer: &mut Buffer, record: &TableRecord) {
        buffer.offset = record.offset as usize;
        let _format = buffer.get::<u16>();
        let _count = buffer.get::<u16>();
        let _string_offset = buffer.get::<Offset16>();
        let mut _names = buffer.get_vec::<Name>(_count as usize);
        let _lang_tag_count: u16 = 0;
        let _lang_tags: Vec<LangTag> = Vec::new();
        let mut table = Table_name {
            _format,
            _count,
            _string_offset,
            _names,
            _lang_tag_count: None,
            _lang_tags: None,
        };
        if _format == 1 {
            let _lang_tag_count = buffer.get::<u16>();
            let _lang_tags = buffer.get_vec::<LangTag>(_lang_tag_count as usize);
            table._lang_tag_count = Some(_lang_tag_count);
            table._lang_tags = Some(_lang_tags);
        };
        table._names.iter_mut().for_each(|x| x.parse(buffer));
        self.name = Some(table);
    }
}

#[derive(Debug)]
struct Name {
    pub platform_id: u16,
    pub encoding_id: u16,
    pub language_id: u16,
    pub name_id: u16,
    _length: u16,
    _offset: Offset16,
    pub string: String,
}

impl Name {
    fn parse(&mut self, buffer: &mut Buffer) {
        let (start, end) = (self._offset, self._offset + self._length);
        let data = buffer.slice(start as usize, end as usize);

        let (cow, _, _) = match (self.platform_id, self.encoding_id) {
            (0, 0)
            | (0, 1)
            | (0, 2)
            | (0, 3)
            | (0, 4)
            | (0, 5)
            | (0, 6)
            | (3, 0)
            | (3, 1)
            | (3, 10) => encoding_rs::UTF_16BE.decode(data),
            (1, 0) => encoding_rs::MACINTOSH.decode(data),
            // (1, 1) is actually CP10001: Apple Japanese (x-mac-japanese)
            (1, 1) | (3, 2) => encoding_rs::SHIFT_JIS.decode(data),
            // (1, 3) is actually CP10003: Apple Korean (x-mac-korean)
            (1, 3) | (3, 5) => encoding_rs::EUC_KR.decode(data),
            (1, 7) | (1, 29) => encoding_rs::X_MAC_CYRILLIC.decode(data),
            (1, 25) => encoding_rs::GBK.decode(data),
            (3, 3) => encoding_rs::GB18030.decode(data),
            (3, 4) => encoding_rs::BIG5.decode(data),
            _ => encoding_rs::UTF_16BE.decode(data),
        };
        // Not check error yet
        self.string.push_str(&cow);
    }
}

impl Read for Name {
    fn read(buffer: &mut Buffer) -> Self {
        Self {
            platform_id: buffer.get::<u16>(),
            encoding_id: buffer.get::<u16>(),
            language_id: buffer.get::<u16>(),
            name_id: buffer.get::<u16>(),
            _length: buffer.get::<u16>(),
            _offset: buffer.get::<Offset16>(),
            string: "".to_string(),
        }
    }
}

#[derive(Debug)]
struct LangTag {
    _length: u16,
    _offset: Offset16,
    pub tag: String,
}

impl Read for LangTag {
    fn read(buffer: &mut Buffer) -> Self {
        let _length = buffer.get::<u16>();
        let _offset = buffer.get::<Offset16>();
        let tag = String::new();
        Self {
            _length,
            _offset,
            tag,
        }
    }
}
