use crate::font::Font;
use crate::util::{Buffer, ReadBuffer};

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
    name: String,
    top_dict: TopDict,
    string: Vec<String>,
}

impl Font {
    #[allow(non_snake_case)]
    pub fn parse_CFF_(&mut self, buffer: &mut Buffer) {
        let cff_start_offset = buffer.offset;
        let _version = buffer.get_version::<u8>();
        let header_size = buffer.get::<u8>();
        let offset_size = buffer.get::<u8>();
        buffer.offset = cff_start_offset + header_size as usize;
        // We assume that the name index only contains 1 element.
        let name = String::from_utf8(
            buffer.get::<Index>().data.first().unwrap().to_vec()).unwrap();
        let mut top_dict = TopDict {
            _data: buffer.get::<Index>().data.first().unwrap().to_vec(),
            ..Default::default()
        };
        let string = buffer.get::<Index>().to_string_vec();
        top_dict.parse(&string);
        self.CFF_ = Some(Table_CFF_ {
            _version,
            header_size,
            offset_size,
            name,
            top_dict,
            string,
        });
    }
}

#[derive(Debug)]
struct TopDict {
    _data: Vec<u8>,
    version: String,
    notice: String,
    copyright: String,
    full_name: String,
    family_name: String,
    weight: String,
    is_fixed_pitch: bool,
    italic_angle: i32,
    underline_position: i32,
    underline_thickness: i32,
    paint_type: i32,
    char_string_type: i32,
    font_matrix: Vec<f32>,
    unique_id: Option<i32>,
    font_bbox: Vec<i32>,
    stroke_width: i32,
    xuid: Option<Vec<i32>>,
    charset: i32,
    encoding: i32,
    char_strings: Option<i32>,
    private: Option<(i32, i32)>,
    synthetic_base: Option<i32>,
    postscript: Option<String>,
    base_font_name: Option<String>,
    base_font_blend: Option<Vec<f32>>,
    cid: Option<CID>,
}

impl TopDict {
    fn _get_num(dict_item: &mut DictItem) -> i32 {
        let num = dict_item.integer.pop().unwrap();
        dict_item.integer.clear();
        num
    }

    fn _get_bool(dict_item: &mut DictItem) -> bool {
        let boolean = dict_item.integer.pop().unwrap() != 0;
        dict_item.integer.clear();
        boolean
    }

    fn _get_string(dict_item: &mut DictItem, strings: &Vec<String>) -> String {
        let index = dict_item.integer.pop().unwrap() as usize;
        dict_item.integer.clear();
        if index < CFF_STANDARD_STRINGS_LEN {
            CFF_STANDARD_STRINGS[index].to_string()
        } else {
            strings[index - CFF_STANDARD_STRINGS_LEN].to_string()
        }
    }

    fn _get_version(dict_item: &mut DictItem) -> String {
        if !dict_item.integer.is_empty() {
            let num = dict_item.integer.pop().unwrap();
            dict_item.integer.clear();
            num.to_string()
        } else if !dict_item.real.is_empty() {
            let num_str = dict_item.real.pop().unwrap();
            dict_item.real.clear();
            num_str
        } else {
            Default::default()
        }
    }

    fn _get_integer_array(dict_item: &mut DictItem) -> Vec<i32> {
        let array = dict_item.integer.to_vec();
        dict_item.integer.clear();
        array
    }

    fn _get_real_array(dict_item: &mut DictItem) -> Vec<f32> {
        let array = dict_item.real.iter().map(|i| i.parse().unwrap()).collect();
        dict_item.real.clear();
        array
    }

    fn parse(&mut self, strings: &Vec<String>) {
        let mut i = 0;
        let mut item = DictItem::default();
        let mut is_cid = false;
        let mut cid = CID::default();
        while i < self._data.len() {
            let b0 = self._data[i] as i32;
            match b0 {
                // Operators
                0 => self.version = Self::_get_string(&mut item, strings),
                1 => self.notice = Self::_get_string(&mut item, strings),
                2 => self.full_name = Self::_get_string(&mut item, strings),
                3 => self.family_name = Self::_get_string(&mut item, strings),
                4 => self.weight = Self::_get_string(&mut item, strings),
                5 => self.font_bbox = Self::_get_integer_array(&mut item),
                12 => {
                    match self._data[i + 1] {
                        0 => self.copyright = Self::_get_string(&mut item, strings),
                        1 => self.is_fixed_pitch = Self::_get_bool(&mut item),
                        2 => self.italic_angle = Self::_get_num(&mut item),
                        3 => self.underline_position = Self::_get_num(&mut item),
                        4 => self.underline_thickness = Self::_get_num(&mut item),
                        5 => self.paint_type = Self::_get_num(&mut item),
                        6 => self.char_string_type = Self::_get_num(&mut item),
                        7 => self.font_matrix = Self::_get_real_array(&mut item),
                        8 => self.stroke_width = Self::_get_num(&mut item),
                        20 => self.synthetic_base = Some(Self::_get_num(&mut item)),
                        21 => self.postscript = Some(Self::_get_string(&mut item, strings)),
                        22 => self.base_font_name = Some(Self::_get_string(&mut item, strings)),
                        23 => self.base_font_blend = Some(Self::_get_real_array(&mut item)),
                        30 => {
                            is_cid = true;
                            let num = item.integer.pop().unwrap();
                            let index2 = item.integer.pop().unwrap() as usize;
                            let s2 = if index2 < CFF_STANDARD_STRINGS_LEN {
                                CFF_STANDARD_STRINGS[index2].to_string()
                            } else {
                                strings[index2 - CFF_STANDARD_STRINGS_LEN].to_string()
                            };
                            let index1 = item.integer.pop().unwrap() as usize;
                            let s1 = if index1 < CFF_STANDARD_STRINGS_LEN {
                                CFF_STANDARD_STRINGS[index1].to_string()
                            } else {
                                strings[index1 - CFF_STANDARD_STRINGS_LEN].to_string()
                            };
                            item.integer.clear();
                            cid.ros = (s1, s2, num);
                        },
                        31 => {
                            is_cid = true;
                            cid.cid_font_version = Self::_get_version(&mut item);
                        }
                        32 => {
                            is_cid = true;
                            cid.cid_font_revision = Self::_get_version(&mut item);
                        }
                        33 => {
                            is_cid = true;
                            cid.cid_font_type = Self::_get_num(&mut item);
                        }
                        34 => {
                            is_cid = true;
                            cid.cid_count = Self::_get_num(&mut item);
                        }
                        35 => {
                            is_cid = true;
                            cid.uid_base = Self::_get_num(&mut item);
                        }
                        36 => {
                            is_cid = true;
                            cid.fd_array = Self::_get_num(&mut item);
                        }
                        37 => {
                            is_cid = true;
                            cid.fd_select = Self::_get_num(&mut item);
                        }
                        38 => {
                            is_cid = true;
                            cid.font_name = Self::_get_string(&mut item, strings);
                        }
                        _ => println!("[DEBUG] \"{}:{}\" 12 {}", file!(), line!(), self._data[i + 1]),
                    }
                    i += 1;
                }
                13 => self.unique_id = Some(Self::_get_num(&mut item)),
                14 => self.xuid = Some(Self::_get_integer_array(&mut item)),
                15 => self.charset = Self::_get_num(&mut item),
                16 => self.encoding = Self::_get_num(&mut item),
                17 => self.char_strings = Some(Self::_get_num(&mut item)),
                18 => self.private = {
                    let num2 = item.integer.pop().unwrap();
                    let num1 = item.integer.pop().unwrap();
                    item.integer.clear();
                    Some((num1, num2))
                },
                // Operands: integer
                32..=246 => item.integer.push(b0 - 139),
                247..=250 => {
                    let b1 = self._data[i + 1] as i32;
                    item.integer.push((b0 - 247) * 256 + b1 + 108);
                    i += 1;
                },
                251..=254 => {
                    let b1 = self._data[i + 1] as i32;
                    item.integer.push(-(b0 - 251) * 256 - b1 - 108);
                    i += 1;
                },
                28 => {
                    let b1 = self._data[i + 1] as i32;
                    let b2 = self._data[i + 2] as i32;
                    item.integer.push(b1 << 8 | b2);
                    i += 2;
                },
                29 => {
                    let b1 = self._data[i + 1] as i32;
                    let b2 = self._data[i + 2] as i32;
                    let b3 = self._data[i + 3] as i32;
                    let b4 = self._data[i + 4] as i32;
                    item.integer.push(b1 << 24 | b2 << 16 | b3 << 8 | b4);
                    i += 4;
                },
                // Operands: real
                30 => {
                    let mut s = String::new();
                    loop {
                        macro_rules! _match_nibble {
                            ($nibble:expr) => {
                                match $nibble {
                                    0..=9 => s += &$nibble.to_string(),
                                    0xA => s += ".",
                                    0xB => s += "e",
                                    0xC => s += "e-",
                                    0xE => s += "-",
                                    0xF => {
                                        item.real.push(s.parse().unwrap());
                                        i += 1;
                                        break;
                                    }
                                    _ => unreachable!(),
                                }
                            };
                        }
                        let b1 = self._data[i + 1];
                        _match_nibble!(b1 >> 4);
                        _match_nibble!((b1 << 4) >> 4);
                        i += 1;
                    }
                }
                _ => println!("[DEBUG] \"{}:{}\" {}", file!(), line!(), b0),
            }
            i += 1;
        }
        if is_cid {
            self.cid = Some(cid);
        }
    }
}

impl Default for TopDict {
    fn default() -> Self {
        Self {
            _data: Default::default(),
            version: Default::default(),
            notice: Default::default(),
            copyright: Default::default(),
            full_name: Default::default(),
            family_name: Default::default(),
            weight: Default::default(),
            is_fixed_pitch: false,
            italic_angle: 0,
            underline_position: -100,
            underline_thickness: 50,
            paint_type: 0,
            char_string_type: 2,
            font_matrix: vec![0.001, 0.0, 0.001, 0.0],
            unique_id: Default::default(),
            font_bbox: vec![0, 0, 0, 0],
            stroke_width: 0,
            xuid: Default::default(),
            charset: 0,
            encoding: 0,
            char_strings: Default::default(),
            private: Default::default(),
            synthetic_base: Default::default(),
            postscript: Default::default(),
            base_font_name: Default::default(),
            base_font_blend: Default::default(),
            cid: Default::default(),
        }
    }
}

#[derive(Debug)]
struct CID {
    ros: (String, String, i32),
    cid_font_version: String,
    cid_font_revision: String,
    cid_font_type: i32,
    cid_count: i32,
    uid_base: i32,
    fd_array: i32,
    fd_select: i32,
    font_name: String,
}

impl Default for CID {
    fn default() -> Self {
        Self {
            ros: Default::default(),
            cid_font_version: "0".to_string(),
            cid_font_revision: "0".to_string(),
            cid_font_type: 0,
            cid_count: 8720,
            uid_base: Default::default(),
            fd_array: Default::default(),
            fd_select: Default::default(),
            font_name: Default::default(),
        }
    }
}

#[derive(Debug, Default)]
struct DictItem {
    // TODO: use a "number" type (maybe union?)
    integer: Vec<i32>,
    real: Vec<String>,
    boolean: Option<bool>,
}

/// An array of variable-sized objects.
#[derive(Debug, Default)]
struct Index {
    count: usize,  // Actual type is `u16`
    offset_size: u8,
    offset: Vec<usize>,  // Actual type is `Offset[]`
    data: Vec<Vec<u8>>,
}

impl Index {
    fn to_string_vec(&self) -> Vec<String> {
        self.data
            .iter()
            .map(|i| String::from_utf8(i.to_vec()).unwrap())
            .collect()
    }
}

impl ReadBuffer for Index {
    fn read(buffer: &mut Buffer) -> Self {
        let count = buffer.get::<u16>() as usize;
        if count == 0 {
            Self {
                count,
                ..Default::default()
            }
        } else {
            let offset_size = buffer.get::<u8>();
            macro_rules! _get_offset {
                ($t:ty) => {
                    buffer.get_vec::<$t>(count + 1).iter().map(|&i| i as usize).collect()
                }
            }
            let offset: Vec<usize> = match offset_size {
                1 => _get_offset!(u8),
                2 => _get_offset!(u16),
                3 => _get_offset!(u32),
                4 => _get_offset!(u64),
                _ => unreachable!(),
            };
            let data = (0..count)
                .map(|i| buffer.get_vec::<u8>(offset[i + 1] - offset[i]))
                .collect();
            Self {
                count,
                offset_size,
                offset,
                data,
            }
        }
    }
}

const CFF_STANDARD_STRINGS_LEN: usize = 391;
const CFF_STANDARD_STRINGS: [&str; CFF_STANDARD_STRINGS_LEN] = [
    ".notdef",
    "space",
    "exclam",
    "quotedbl",
    "numbersign",
    "dollar",
    "percent",
    "ampersand",
    "quoteright",
    "parenleft",
    "parenright",
    "asterisk",
    "plus",
    "comma",
    "hyphen",
    "period",
    "slash",
    "zero",
    "one",
    "two",
    "three",
    "four",
    "five",
    "six",
    "seven",
    "eight",
    "nine",
    "colon",
    "semicolon",
    "less",
    "equal",
    "greater",
    "question",
    "at",
    "A",
    "B",
    "C",
    "D",
    "E",
    "F",
    "G",
    "H",
    "I",
    "J",
    "K",
    "L",
    "M",
    "N",
    "O",
    "P",
    "Q",
    "R",
    "S",
    "T",
    "U",
    "V",
    "W",
    "X",
    "Y",
    "Z",
    "bracketleft",
    "backslash",
    "bracketright",
    "asciicircum",
    "underscore",
    "quoteleft",
    "a",
    "b",
    "c",
    "d",
    "e",
    "f",
    "g",
    "h",
    "i",
    "j",
    "k",
    "l",
    "m",
    "n",
    "o",
    "p",
    "q",
    "r",
    "s",
    "t",
    "u",
    "v",
    "w",
    "x",
    "y",
    "z",
    "braceleft",
    "bar",
    "braceright",
    "asciitilde",
    "exclamdown",
    "cent",
    "sterling",
    "fraction",
    "yen",
    "florin",
    "section",
    "currency",
    "quotesingle",
    "quotedblleft",
    "guillemotleft",
    "guilsinglleft",
    "guilsinglright",
    "fi",
    "fl",
    "endash",
    "dagger",
    "daggerdbl",
    "periodcentered",
    "paragraph",
    "bullet",
    "quotesinglbase",
    "quotedblbase",
    "quotedblright",
    "guillemotright",
    "ellipsis",
    "perthousand",
    "questiondown",
    "grave",
    "acute",
    "circumflex",
    "tilde",
    "macron",
    "breve",
    "dotaccent",
    "dieresis",
    "ring",
    "cedilla",
    "hungarumlaut",
    "ogonek",
    "caron",
    "emdash",
    "AE",
    "ordfeminine",
    "Lslash",
    "Oslash",
    "OE",
    "ordmasculine",
    "ae",
    "dotlessi",
    "lslash",
    "oslash",
    "oe",
    "germandbls",
    "onesuperior",
    "logicalnot",
    "mu",
    "trademark",
    "Eth",
    "onehalf",
    "plusminus",
    "Thorn",
    "onequarter",
    "divide",
    "brokenbar",
    "degree",
    "thorn",
    "threequarters",
    "twosuperior",
    "registered",
    "minus",
    "eth",
    "multiply",
    "threesuperior",
    "copyright",
    "Aacute",
    "Acircumflex",
    "Adieresis",
    "Agrave",
    "Aring",
    "Atilde",
    "Ccedilla",
    "Eacute",
    "Ecircumflex",
    "Edieresis",
    "Egrave",
    "Iacute",
    "Icircumflex",
    "Idieresis",
    "Igrave",
    "Ntilde",
    "Oacute",
    "Ocircumflex",
    "Odieresis",
    "Ograve",
    "Otilde",
    "Scaron",
    "Uacute",
    "Ucircumflex",
    "Udieresis",
    "Ugrave",
    "Yacute",
    "Ydieresis",
    "Zcaron",
    "aacute",
    "acircumflex",
    "adieresis",
    "agrave",
    "aring",
    "atilde",
    "ccedilla",
    "eacute",
    "ecircumflex",
    "edieresis",
    "egrave",
    "iacute",
    "icircumflex",
    "idieresis",
    "igrave",
    "ntilde",
    "oacute",
    "ocircumflex",
    "odieresis",
    "ograve",
    "otilde",
    "scaron",
    "uacute",
    "ucircumflex",
    "udieresis",
    "ugrave",
    "yacute",
    "ydieresis",
    "zcaron",
    "exclamsmall",
    "Hungarumlautsmall",
    "dollaroldstyle",
    "dollarsuperior",
    "ampersandsmall",
    "Acutesmall",
    "parenleftsuperior",
    "parenrightsuperior",
    "twodotenleader",
    "onedotenleader",
    "zerooldstyle",
    "oneoldstyle",
    "twooldstyle",
    "threeoldstyle",
    "fouroldstyle",
    "fiveoldstyle",
    "sixoldstyle",
    "sevenoldstyle",
    "eightoldstyle",
    "nineoldstyle",
    "commasuperior",
    "threequartersemdash",
    "periodsuperior",
    "questionsmall",
    "asuperior",
    "bsuperior",
    "centsuperior",
    "dsuperior",
    "esuperior",
    "isuperior",
    "lsuperior",
    "msuperior",
    "nsuperior",
    "osuperior",
    "rsuperior",
    "ssuperior",
    "tsuperior",
    "ff",
    "ffi",
    "ffl",
    "parenleftinferior",
    "parenrightinferior",
    "Circumflexsmall",
    "hyphensuperior",
    "Gravesmall",
    "Asmall",
    "Bsmall",
    "Csmall",
    "Dsmall",
    "Esmall",
    "Fsmall",
    "Gsmall",
    "Hsmall",
    "Ismall",
    "Jsmall",
    "Ksmall",
    "Lsmall",
    "Msmall",
    "Nsmall",
    "Osmall",
    "Psmall",
    "Qsmall",
    "Rsmall",
    "Ssmall",
    "Tsmall",
    "Usmall",
    "Vsmall",
    "Wsmall",
    "Xsmall",
    "Ysmall",
    "Zsmall",
    "colonmonetary",
    "onefitted",
    "rupiah",
    "Tildesmall",
    "exclamdownsmall",
    "centoldstyle",
    "Lslashsmall",
    "Scaronsmall",
    "Zcaronsmall",
    "Dieresissmall",
    "Brevesmall",
    "Caronsmall",
    "Dotaccentsmall",
    "Macronsmall",
    "figuredash",
    "hypheninferior",
    "Ogoneksmall",
    "Ringsmall",
    "Cedillasmall",
    "questiondownsmall",
    "oneeighth",
    "threeeighths",
    "fiveeighths",
    "seveneighths",
    "onethird",
    "twothirds",
    "zerosuperior",
    "foursuperior",
    "fivesuperior",
    "sixsuperior",
    "sevensuperior",
    "eightsuperior",
    "ninesuperior",
    "zeroinferior",
    "oneinferior",
    "twoinferior",
    "threeinferior",
    "fourinferior",
    "fiveinferior",
    "sixinferior",
    "seveninferior",
    "eightinferior",
    "nineinferior",
    "centinferior",
    "dollarinferior",
    "periodinferior",
    "commainferior",
    "Agravesmall",
    "Aacutesmall",
    "Acircumflexsmall",
    "Atildesmall",
    "Adieresissmall",
    "Aringsmall",
    "AEsmall",
    "Ccedillasmall",
    "Egravesmall",
    "Eacutesmall",
    "Ecircumflexsmall",
    "Edieresissmall",
    "Igravesmall",
    "Iacutesmall",
    "Icircumflexsmall",
    "Idieresissmall",
    "Ethsmall",
    "Ntildesmall",
    "Ogravesmall",
    "Oacutesmall",
    "Ocircumflexsmall",
    "Otildesmall",
    "Odieresissmall",
    "OEsmall",
    "Oslashsmall",
    "Ugravesmall",
    "Uacutesmall",
    "Ucircumflexsmall",
    "Udieresissmall",
    "Yacutesmall",
    "Thornsmall",
    "Ydieresissmall",
    "001.000",
    "001.001",
    "001.002",
    "001.003",
    "Black",
    "Bold",
    "Book",
    "Light",
    "Medium",
    "Regular",
    "Roman",
    "Semibold",
];
