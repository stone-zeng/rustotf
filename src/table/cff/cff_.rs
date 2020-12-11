use std::fmt;
use crate::font::Font;
use crate::util::{Buffer, ReadBuffer, u24};

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
    _header_size: u8,
    _offset_size: u8,
    name: String,
    top_dict: TopDict,
    strings: Vec<String>,
    // encodings: Encoding
    charsets: Vec<String>,
    char_strings: Index,
}

impl Font {
    #[allow(non_snake_case)]
    pub fn parse_CFF_(&mut self, buffer: &mut Buffer) {
        let cff_start_offset = buffer.offset;
        let _version = buffer.get_version::<u8>();
        let _header_size = buffer.get();
        let _offset_size = buffer.get();
        buffer.offset = cff_start_offset + _header_size as usize;
        // We assume that the name index only contains 1 element.
        let name = String::from_utf8(buffer.get::<Index>().data[0].to_vec()).unwrap();
        let top_dict_data = buffer.get::<Index>().data[0].to_vec();
        let strings = buffer.get::<Index>().to_string_vec();
        let top_dict = TopDict::new(top_dict_data, &strings);
        // TODO: parser not implemented
        let char_strings = match top_dict.char_strings {
            Some(offset) => {
                buffer.offset = cff_start_offset + offset as usize;
                buffer.get::<Index>()
            },
            _ => Default::default(),
        };
        let num_glyphs = char_strings.count;
        let charsets = match top_dict.charset {
            0 => unimplemented!("Charset: ISOAdobe"),
            1 => unimplemented!("Charset: Expert"),
            2 => unimplemented!("Charset: ExpertSubset"),
            offset => {
                macro_rules! _get_charsets {
                    ($t:ty) => {{
                        // ".notdef" is omitted in the array.
                        let mut count = 1;
                        let mut result = vec![CFF_STD_STRINGS[0].to_string()];
                        while count < num_glyphs {
                            let sid = buffer.get::<u16>() as usize;
                            let num_left = buffer.get::<$t>() as usize;
                            (0..=num_left).for_each(|i| result.push((sid + i).to_string()));
                            count += num_left as usize + 1;
                        }
                        result
                    }};
                };
                buffer.offset = cff_start_offset + offset as usize;
                let format: u8 = buffer.get();
                match format {
                    0 => (0..num_glyphs)
                        .map(|_| from_sid(buffer.get::<u16>() as usize, &strings))
                        .collect(),
                    1 => _get_charsets!(u8),
                    2 => _get_charsets!(u16),
                    _ => unreachable!(),
                }
            }
        };
        self.CFF_ = Some(Table_CFF_ {
            _version,
            _header_size,
            _offset_size,
            name,
            top_dict,
            strings,
            charsets,
            char_strings: Default::default(),
        });
    }
}

#[derive(Debug)]
struct TopDict {
    version: String,
    notice: String,
    copyright: String,
    full_name: String,
    family_name: String,
    weight: String,
    is_fixed_pitch: bool,
    italic_angle: Number,
    underline_position: Number,
    underline_thickness: Number,
    paint_type: i32,
    char_string_type: i32,
    font_matrix: Vec<Number>,
    unique_id: Option<i32>,
    font_bbox: Vec<Number>,
    stroke_width: Number,
    xuid: Option<Vec<Number>>,
    charset: i32,
    encoding: i32,
    char_strings: Option<i32>,
    private: Option<(i32, i32)>,
    synthetic_base: Option<i32>,
    postscript: Option<String>,
    base_font_name: Option<String>,
    base_font_blend: Option<Vec<Number>>,
    cid: Option<CID>,
}

impl TopDict {
    fn new(data: Vec<u8>, strings: &Vec<String>) -> Self {
        let mut i = 0;
        let mut temp = Vec::new();
        let mut dict = TopDict::default();
        let mut cid = CID::default();
        let mut is_cid = false;

        let get_num = |nums: &mut Vec<Number>| {
            let num = nums.pop().unwrap();
            nums.clear();
            num
        };

        let get_bool = |nums: &mut Vec<Number>| {
            get_num(nums).integer() != 0
        };

        let get_array = |nums: &mut Vec<Number>| {
            let nums_copy = nums.to_vec();
            nums.clear();
            nums_copy
        };

        let get_string = |nums: &mut Vec<Number>| {
            let num = nums.pop().unwrap();
            nums.clear();
            from_sid(num.integer() as usize, strings)
        };

        let get_private = |nums: &mut Vec<Number>| {
            let num2 = nums.pop().unwrap().integer();
            let num1 = nums.pop().unwrap().integer();
            nums.clear();
            (num1, num2)
        };

        let get_ros = |nums: &mut Vec<Number>| {
            let supplement = nums.pop().unwrap().integer();
            let index2 = nums.pop().unwrap().integer() as usize;
            let index1 = nums.pop().unwrap().integer() as usize;
            (from_sid(index1, strings), from_sid(index2, strings), supplement)
        };

        macro_rules! _update_cid {
            ($i:ident, $e:expr) => {{
                is_cid = true;
                cid.$i = $e
            }}
        }

        while i < data.len() {
            let b0 = data[i];
            match b0 {
                // Operators
                0 => dict.version = get_string(&mut temp),
                1 => dict.notice = get_string(&mut temp),
                2 => dict.full_name = get_string(&mut temp),
                3 => dict.family_name = get_string(&mut temp),
                4 => dict.weight = get_string(&mut temp),
                5 => dict.font_bbox = get_array(&mut temp),
                12 => {
                    let b1 = data[i + 1];
                    match b1 {
                        0 => dict.copyright = get_string(&mut temp),
                        1 => dict.is_fixed_pitch = get_bool(&mut temp),
                        2 => dict.italic_angle = get_num(&mut temp),
                        3 => dict.underline_position = get_num(&mut temp),
                        4 => dict.underline_thickness = get_num(&mut temp),
                        5 => dict.paint_type = get_num(&mut temp).integer(),
                        6 => dict.char_string_type = get_num(&mut temp).integer(),
                        7 => dict.font_matrix = get_array(&mut temp),
                        8 => dict.stroke_width = get_num(&mut temp),
                        20 => dict.synthetic_base = Some(get_num(&mut temp).integer()),
                        21 => dict.postscript = Some(get_string(&mut temp)),
                        22 => dict.base_font_name = Some(get_string(&mut temp)),
                        23 => dict.base_font_blend = Some(get_array(&mut temp)),
                        30 => _update_cid!(ros, get_ros(&mut temp)),
                        31 => _update_cid!(cid_font_version, get_num(&mut temp)),
                        32 => _update_cid!(cid_font_revision, get_num(&mut temp)),
                        33 => _update_cid!(cid_font_type, get_num(&mut temp).integer()),
                        34 => _update_cid!(cid_count, get_num(&mut temp).integer()),
                        35 => _update_cid!(uid_base, get_num(&mut temp).integer()),
                        36 => _update_cid!(fd_array, get_num(&mut temp).integer()),
                        37 => _update_cid!(fd_select, get_num(&mut temp).integer()),
                        38 => _update_cid!(font_name, get_string(&mut temp)),
                        _ => println!("[DEBUG] \"{}:{}\" 12 {}", file!(), line!(), data[i + 1]),
                    }
                    i += 1;
                }
                13 => dict.unique_id = Some(get_num(&mut temp).integer()),
                14 => dict.xuid = Some(get_array(&mut temp)),
                15 => dict.charset = get_num(&mut temp).integer(),
                16 => dict.encoding = get_num(&mut temp).integer(),
                17 => dict.char_strings = Some(get_num(&mut temp).integer()),
                18 => dict.private = Some(get_private(&mut temp)),
                // Operands: integer
                32..=246 => {
                    let b0 = b0 as i32;
                    temp.push(Number::Integer(b0 - 139))
                },
                247..=250 => {
                    let b0 = b0 as i32;
                    let b1 = data[i + 1] as i32;
                    temp.push(Number::Integer((b0 - 247) * 256 + b1 + 108));
                    i += 1;
                },
                251..=254 => {
                    let b0 = b0 as i32;
                    let b1 = data[i + 1] as i32;
                    temp.push(Number::Integer(-(b0 - 251) * 256 - b1 - 108));
                    i += 1;
                },
                28 => {
                    let b1 = data[i + 1] as i16;
                    let b2 = data[i + 2] as i16;
                    temp.push(Number::Integer((b1 << 8 | b2) as i32));
                    i += 2;
                },
                29 => {
                    let b1 = data[i + 1] as i32;
                    let b2 = data[i + 2] as i32;
                    let b3 = data[i + 3] as i32;
                    let b4 = data[i + 4] as i32;
                    temp.push(Number::Integer(b1 << 24 | b2 << 16 | b3 << 8 | b4));
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
                                        temp.push(Number::Real(s));
                                        i += 1;
                                        break;
                                    }
                                    _ => unreachable!(),
                                }
                            };
                        }
                        let b1 = data[i + 1];
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
            dict.cid = Some(cid);
        }
        dict
    }
}

impl Default for TopDict {
    fn default() -> Self {
        Self {
            version: Default::default(),
            notice: Default::default(),
            copyright: Default::default(),
            full_name: Default::default(),
            family_name: Default::default(),
            weight: Default::default(),
            is_fixed_pitch: false,
            italic_angle: Number::Integer(0),
            underline_position: Number::Integer(-100),
            underline_thickness: Number::Integer(50),
            paint_type: 0,
            char_string_type: 2,
            font_matrix: vec![
                Number::Real((0.001).to_string()),
                Number::Real((0.0).to_string()),
                Number::Real((0.001).to_string()),
                Number::Real((0.0).to_string()),
            ],
            unique_id: Default::default(),
            font_bbox: vec![
                Number::Integer(0),
                Number::Integer(0),
                Number::Integer(0),
                Number::Integer(0),
            ],
            stroke_width: Number::Integer(0),
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
    cid_font_version: Number,
    cid_font_revision: Number,
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
            cid_font_version: Number::Integer(0),
            cid_font_revision: Number::Integer(0),
            cid_font_type: 0,
            cid_count: 8720,
            uid_base: Default::default(),
            fd_array: Default::default(),
            fd_select: Default::default(),
            font_name: Default::default(),
        }
    }
}

#[derive(Clone)]
enum Number {
    Integer(i32),
    Real(String),
}

impl Number {
    fn integer(self) -> i32 {
        match self {
            Self::Integer(n) => n,
            Self::Real(_) => panic!(),
        }
    }
}

impl fmt::Debug for Number {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Integer(n) => write!(f, "{}", n),
            Self::Real(n) => write!(f, "{}", n),
        }
    }
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
            let offset_size = buffer.get();
            macro_rules! _get_offset {
                ($t:ty) => {
                    buffer.get_vec::<$t>(count + 1).iter().map(|&i| i as usize).collect()
                }
            }
            let offset: Vec<usize> = match offset_size {
                1 => _get_offset!(u8),
                2 => _get_offset!(u16),
                // u24 is not primitive type.
                3 => buffer.get_vec::<u24>(count + 1).iter().map(|&i| usize::from(i)).collect(),
                4 => _get_offset!(u32),
                _ => unreachable!(),
            };
            let data = (0..count)
                .map(|i| buffer.get_vec(offset[i + 1] - offset[i]))
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

fn from_sid(sid: usize, strings: &Vec<String>) -> String {
    if sid < CFF_STD_STRINGS_LEN {
        CFF_STD_STRINGS[sid].to_string()
    } else {
        strings[sid - CFF_STD_STRINGS_LEN].to_string()
    }
}

const CFF_STD_STRINGS_LEN: usize = 391;
const CFF_STD_STRINGS: [&str; CFF_STD_STRINGS_LEN] = [
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
