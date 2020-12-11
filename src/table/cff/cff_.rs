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
    synthetic_base: Option<i32>,
    postscript: Option<String>,
    base_font_name: Option<String>,
    base_font_blend: Option<Delta>,

    charset: i32,
    encoding: i32,
    char_strings: i32,

    _private_size: usize,
    _private_offset: usize,
    private: Option<Private>,
    cid: Option<CID>,

    // top_dict: TopDict,
    // strings: Vec<String>,
    // encodings: Encoding
    // charsets: Vec<String>,
    // char_strings: Index,
}

impl Table_CFF_ {
    fn parse_top_dict(&mut self, data: &Vec<u8>, strings: &Vec<String>) {
        let mut i = 0;
        let mut temp = Vec::new();
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
            let num2 = nums.pop().unwrap().integer() as usize;
            let num1 = nums.pop().unwrap().integer() as usize;
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
                0 => self.version = get_string(&mut temp),
                1 => self.notice = get_string(&mut temp),
                2 => self.full_name = get_string(&mut temp),
                3 => self.family_name = get_string(&mut temp),
                4 => self.weight = get_string(&mut temp),
                5 => self.font_bbox = get_array(&mut temp),
                12 => {
                    let b1 = data[i + 1];
                    match b1 {
                        0 => self.copyright = get_string(&mut temp),
                        1 => self.is_fixed_pitch = get_bool(&mut temp),
                        2 => self.italic_angle = get_num(&mut temp),
                        3 => self.underline_position = get_num(&mut temp),
                        4 => self.underline_thickness = get_num(&mut temp),
                        5 => self.paint_type = get_num(&mut temp).integer(),
                        6 => self.char_string_type = get_num(&mut temp).integer(),
                        7 => self.font_matrix = get_array(&mut temp),
                        8 => self.stroke_width = get_num(&mut temp),
                        20 => self.synthetic_base = Some(get_num(&mut temp).integer()),
                        21 => self.postscript = Some(get_string(&mut temp)),
                        22 => self.base_font_name = Some(get_string(&mut temp)),
                        23 => self.base_font_blend = Some(Delta::new(get_array(&mut temp))),
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
                13 => self.unique_id = Some(get_num(&mut temp).integer()),
                14 => self.xuid = Some(get_array(&mut temp)),
                15 => self.charset = get_num(&mut temp).integer(),
                16 => self.encoding = get_num(&mut temp).integer(),
                17 => {} // TODO: self.char_strings = Some(get_num(&mut temp).integer()),
                18 => {
                    let private = get_private(&mut temp);
                    self._private_size = private.0;
                    self._private_offset = private.1;
                },
                // Operands
                30 => temp.push(Self::get_real(&data, &mut i)),
                _ => temp.push(Self::get_integer(&data, &mut i, b0)),
            }
            i += 1;
        }

        if is_cid {
            self.cid = Some(cid);
        }
    }

    fn parse_private_dict(&mut self, data: &Vec<u8>) {
        let mut i = 0;
        let mut temp = Vec::new();
        let mut private = Private::default();

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

        while i < data.len() {
            let b0 = data[i];
            match b0 {
                6 => private.blue_values = Delta::new(get_array(&mut temp)),
                7 => private.other_blues = Delta::new(get_array(&mut temp)),
                8 => private.family_blues = Delta::new(get_array(&mut temp)),
                9 => private.family_other_blues = Delta::new(get_array(&mut temp)),
                10 => private.std_hw = get_num(&mut temp),
                11 => private.std_vw = get_num(&mut temp),
                12 => {
                    let b1 = data[i + 1];
                    match b1 {
                        9 => private.blue_scale = get_num(&mut temp),
                        10 => private.blue_shift = get_num(&mut temp),
                        11 => private.blue_fuzz = get_num(&mut temp),
                        12 => private.stem_snap_h = Delta::new(get_array(&mut temp)),
                        13 => private.stem_snap_v = Delta::new(get_array(&mut temp)),
                        14 => private.force_bold = get_bool(&mut temp),
                        17 => private.language_group = get_num(&mut temp),
                        18 => private.expansion_factor = get_num(&mut temp),
                        19 => private.initial_random_seed = get_num(&mut temp),
                        _ => unreachable!(),
                    }
                    i += 1;
                }
                19 => private.subrs = Some(get_num(&mut temp)),
                20 => private.default_width_x = get_num(&mut temp),
                21 => private.nominal_width_x = get_num(&mut temp),
                // Operands
                30 => temp.push(Self::get_real(&data, &mut i)),
                _ => temp.push(Self::get_integer(&data, &mut i, b0)),
            }
            i += 1;
        }

        self.private = Some(private);
    }

    fn get_real(data: &Vec<u8>, i: &mut usize) -> Number {
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
                            *i += 1;
                            return Number::Real(s);
                        }
                        _ => unreachable!(),
                    }
                };
            }
            let b1 = data[*i + 1];
            _match_nibble!(b1 >> 4);
            _match_nibble!((b1 << 4) >> 4);
            *i += 1;
        }
    }

    fn get_integer(data: &Vec<u8>, i: &mut usize, b0: u8) -> Number {
        match b0 {
            32..=246 => {
                let b0 = b0 as i32;
                Number::Integer(b0 - 139)
            },
            247..=250 => {
                let b0 = b0 as i32;
                let b1 = data[*i + 1] as i32;
                *i += 1;
                Number::Integer((b0 - 247) * 256 + b1 + 108)
            },
            251..=254 => {
                let b0 = b0 as i32;
                let b1 = data[*i + 1] as i32;
                *i += 1;
                Number::Integer(-(b0 - 251) * 256 - b1 - 108)
            },
            28 => {
                let b1 = data[*i + 1] as i16;
                let b2 = data[*i + 2] as i16;
                *i += 2;
                Number::Integer((b1 << 8 | b2) as i32)
            },
            29 => {
                let b1 = data[*i + 1] as i32;
                let b2 = data[*i + 2] as i32;
                let b3 = data[*i + 3] as i32;
                let b4 = data[*i + 4] as i32;
                *i += 4;
                Number::Integer(b1 << 24 | b2 << 16 | b3 << 8 | b4)
            },
            _ => unreachable!(), //println!("[DEBUG] \"{}:{}\" {}", file!(), line!(), b0),
        }
    }
}

impl Default for Table_CFF_ {
    fn default() -> Self {
        Self {
            _version: Default::default(),
            _header_size: Default::default(),
            _offset_size: Default::default(),
            name: Default::default(),
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
            synthetic_base: Default::default(),
            postscript: Default::default(),
            base_font_name: Default::default(),
            base_font_blend: Default::default(),
            charset: 0,
            encoding: 0,
            char_strings: Default::default(),
            _private_size: Default::default(),
            _private_offset: Default::default(),
            private: Default::default(),
            cid: Default::default(),
        }
    }
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

        let mut cff = Table_CFF_ {
            _version,
            _header_size,
            _offset_size,
            name,
            ..Default::default()
        };

        let top_dict_data = buffer.get::<Index>().data[0].to_vec();
        let strings = buffer.get::<Index>().to_string_vec();
        cff.parse_top_dict(&top_dict_data, &strings);

        if cff._private_size != 0 {
            buffer.offset = cff_start_offset + cff._private_offset;
            let private_dict_data: Vec<u8> = buffer.get_vec(cff._private_size);
            cff.parse_private_dict(&private_dict_data);
        }

        // let top_dict = TopDict::new(top_dict_data, &strings);
        // TODO: parser not implemented
        /*
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
        */
        self.CFF_ = Some(cff);
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

#[derive(Debug)]
struct Private {
    _size: usize,
    _offset: usize,
    blue_values: Delta,
    other_blues: Delta,
    family_blues: Delta,
    family_other_blues: Delta,
    blue_scale: Number,
    blue_shift: Number,
    blue_fuzz: Number,
    std_hw: Number,
    std_vw: Number,
    stem_snap_h: Delta,
    stem_snap_v: Delta,
    force_bold: bool,
    language_group: Number,
    expansion_factor: Number,
    initial_random_seed: Number,
    subrs: Option<Number>,
    default_width_x: Number,
    nominal_width_x: Number,
}

impl Default for Private {
    fn default() -> Self {
        Self {
            _size: Default::default(),
            _offset: Default::default(),
            blue_values: Default::default(),
            other_blues: Default::default(),
            family_blues: Default::default(),
            family_other_blues: Default::default(),
            blue_scale: Number::Real((0.039625).to_string()),
            blue_shift: Number::Integer(7),
            blue_fuzz: Number::Integer(1),
            std_hw: Default::default(),
            std_vw: Default::default(),
            stem_snap_h: Default::default(),
            stem_snap_v: Default::default(),
            force_bold: Default::default(),
            language_group: Number::Integer(0),
            expansion_factor: Number::Real((0.06).to_string()),
            initial_random_seed: Number::Integer(0),
            subrs: Default::default(),
            default_width_x: Number::Integer(0),
            nominal_width_x: Number::Integer(0),
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

impl Default for Number {
    fn default() -> Self {
        Number::Integer(0)
    }
}

struct Delta {
    _data: Vec<Number>
}

impl Delta {
    fn new(array: Vec<Number>) -> Self {
        Self {
            _data: array.iter()
                .scan(0, |acc, x| {
                    *acc = *acc + x.to_owned().integer(); Some(Number::Integer(*acc))
                })
                .collect()
        }
    }
}

impl fmt::Debug for Delta {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self._data)
    }
}

impl Default for Delta {
    fn default() -> Self {
        Self {
            _data: Default::default()
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
