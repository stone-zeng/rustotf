use std::fmt;

use crate::font::Font;
use crate::table::cff::cff_data::*;
use crate::util::{Buffer, ReadBuffer, u24};

/// ## `CFF` &mdash; Compact Font Format table
///
/// Specification: <https://docs.microsoft.com/en-us/typography/opentype/spec/cff>.
///
/// This table contains a Compact Font Format font representation (also known as a PostScript
/// Type 1, or CIDFont) and is structured according to
/// [*Adobe Technical Note #5176: The Compact Font Format Specification*](https://wwwimages2.adobe.com/content/dam/acom/en/devnet/font/pdfs/5176.CFF.pdf)
/// and [*Adobe Technical Note #5177: Type 2 Charstring Format*](https://wwwimages2.adobe.com/content/dam/acom/en/devnet/font/pdfs/5177.Type2.pdf).

impl Font {
    #[allow(non_snake_case)]
    pub fn parse_CFF_(&mut self, buffer: &mut Buffer) {
        let cff_start_offset = buffer.offset;
        let _version = buffer.get_version::<u8>();
        let _header_size = buffer.get();
        let _offset_size = buffer.get();
        buffer.offset = cff_start_offset + _header_size as usize;
        // We assume that the fontset contains only one element.
        let name = String::from_utf8(buffer.get::<Index>().data[0].to_vec()).unwrap();
        let mut cff = Table_CFF_ {
            _version,
            _header_size,
            _offset_size,
            name,
            ..Default::default()
        };
        // Top dict
        let top_dict_index_data = buffer.get::<Index>().data[0].to_vec();
        let string_index_data = buffer.get::<Index>().to_string_vec();
        cff.parse_top_dict(&top_dict_index_data, &string_index_data);
        // Global subroutines
        cff.global_subr = buffer.get::<Index>().data;
        // Encoding
        cff.encoding = match cff._encoding_offset {
            0 => Encoding::Standard,
            1 => Encoding::Expert,
            _ => {
                buffer.offset = cff_start_offset + cff._encoding_offset;
                buffer.get()
            }
        };
        // Char strings
        buffer.offset = cff_start_offset + cff._char_strings_offset;
        let char_strings_index = buffer.get::<Index>();
        let num_glyphs = char_strings_index.count;
        cff.char_strings = char_strings_index.data;
        // Charset
        macro_rules! _get_charsets {
            ($t:ty) => ({
                // ".notdef" is omitted in the array.
                let mut count = 1;
                let mut result = vec![CFF_STANDARD_STRINGS[0].to_string()];
                while count < num_glyphs {
                    let sid = buffer.get::<u16>() as usize;
                    let num_left = buffer.get::<$t>() as usize;
                    (0..=num_left).for_each(|i| {
                        result.push(from_sid(sid + i, &string_index_data))
                    });
                    count += num_left as usize + 1;
                }
                result
            });
        }
        cff.charset = match cff._charset_offset {
            0 => CFF_ISO_ADOBE_CHARSET.iter().map(|&i| i.to_string()).collect(),
            1 => CFF_EXPERT_CHARSET.iter().map(|&i| i.to_string()).collect(),
            2 => CFF_EXPERT_SUBSET_CHARSET.iter().map(|&i| i.to_string()).collect(),
            offset => {
                buffer.offset = cff_start_offset + offset as usize;
                let format: u8 = buffer.get();
                match format {
                    0 => (0..num_glyphs)
                        .map(|_| from_sid(buffer.get::<u16>() as usize, &string_index_data))
                        .collect(),
                    1 => _get_charsets!(u8),
                    2 => _get_charsets!(u16),
                    _ => unreachable!(),
                }
            }
        };
        // Private dict
        if cff._private_size != 0 {
            buffer.offset = cff_start_offset + cff._private_offset;
            let private_dict_index_data: Vec<u8> = buffer.get_vec(cff._private_size);
            cff.parse_private_dict(&private_dict_index_data);
        }
        self.CFF_ = Some(cff);
    }
}

#[allow(non_camel_case_types)]
#[derive(Debug)]
pub struct Table_CFF_ {
    // Header
    _version: String,
    _header_size: u8,
    _offset_size: u8,
    // Name
    name: String,
    // Top dict
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
    // Encodings
    _encoding_offset: usize,
    encoding: Encoding,
    // Charsets
    _charset_offset: usize,
    charset: Vec<String>,
    // Char strings
    _char_strings_offset: usize,
    char_strings: Vec<Vec<u8>>,
    // Private dict
    _private_size: usize,
    _private_offset: usize,
    private: Option<Private>,
    // CID
    ros: Option<ROS>,
    cid_font_version: Option<Number>,
    cid_font_revision: Option<Number>,
    cid_font_type: Option<i32>,
    cid_count: Option<i32>,
    uid_base: Option<i32>,
    _fd_array_offset: Option<usize>,
    _fd_select_offset: Option<usize>,
    cid_font_name: Option<String>,
    // Global subroutines
    global_subr: Vec<Vec<u8>>,
}

impl Table_CFF_ {
    fn parse_top_dict(&mut self, data: &Vec<u8>, strings: &Vec<String>) {
        let mut i = 0;
        let mut temp: Vec<Number> = Vec::new();

        macro_rules! _pop_num { () => { temp.pop().unwrap() }; }
        macro_rules! _pop_integer { () => { _pop_num!().integer() }; }
        macro_rules! _pop_bool { () => { _pop_integer!() != 0 }; }
        macro_rules! _pop_string { () => { from_sid(_pop_integer!() as usize, strings) }; }
        macro_rules! _pop_array {
            () => ({
                let nums_copy = temp.to_vec();
                temp.clear();
                nums_copy
            });
        }
        macro_rules! _pop_delta { () => { Delta::new(_pop_array!()) }; }
        macro_rules! _pop_private {
            () => ({
                let num2 = temp.pop().unwrap().integer() as usize;
                let num1 = temp.pop().unwrap().integer() as usize;
                (num1, num2)
            });
        }
        macro_rules! _pop_ros {
            () => ({
                let supplement = temp.pop().unwrap().integer();
                let index_o = temp.pop().unwrap().integer() as usize;
                let index_r = temp.pop().unwrap().integer() as usize;
                ROS::new(index_r, index_o, supplement, strings)
            });
        }

        while i < data.len() {
            let b0 = data[i];
            match b0 {
                // Operators
                0 => self.version = _pop_string!(),
                1 => self.notice = _pop_string!(),
                2 => self.full_name = _pop_string!(),
                3 => self.family_name = _pop_string!(),
                4 => self.weight = _pop_string!(),
                5 => self.font_bbox = _pop_array!(),
                12 => {
                    let b1 = data[i + 1];
                    match b1 {
                        0 => self.copyright = _pop_string!(),
                        1 => self.is_fixed_pitch = _pop_bool!(),
                        2 => self.italic_angle = _pop_num!(),
                        3 => self.underline_position = _pop_num!(),
                        4 => self.underline_thickness =_pop_num!(),
                        5 => self.paint_type = _pop_integer!(),
                        6 => self.char_string_type = _pop_integer!(),
                        7 => self.font_matrix = _pop_array!(),
                        8 => self.stroke_width =_pop_num!(),
                        20 => self.synthetic_base = Some(_pop_integer!()),
                        21 => self.postscript = Some(_pop_string!()),
                        22 => self.base_font_name = Some(_pop_string!()),
                        23 => self.base_font_blend = Some(_pop_delta!()),
                        30 => self.ros = Some(_pop_ros!()),
                        31 => self.cid_font_version = Some(_pop_num!()),
                        32 => self.cid_font_revision = Some(_pop_num!()),
                        33 => self.cid_font_type = Some(_pop_integer!()),
                        34 => self.cid_count = Some(_pop_integer!()),
                        35 => self.uid_base = Some(_pop_integer!()),
                        36 => self._fd_array_offset = Some(_pop_integer!() as usize),
                        37 => self._fd_select_offset = Some(_pop_integer!() as usize),
                        38 => self.cid_font_name = Some(_pop_string!()),
                        _ => unreachable!(),
                    }
                    i += 1;
                }
                13 => self.unique_id = Some(_pop_integer!()),
                14 => self.xuid = Some(_pop_array!()),
                15 => self._charset_offset = _pop_integer!() as usize,
                16 => self._encoding_offset = _pop_integer!() as usize,
                17 => self._char_strings_offset = _pop_integer!() as usize,
                18 => {
                    let private = _pop_private!();
                    self._private_size = private.0;
                    self._private_offset = private.1;
                },
                // Operands
                30 => temp.push(Self::get_real(&data, &mut i)),
                _ => temp.push(Self::get_integer(&data, &mut i, b0)),
            }
            i += 1;
        }

        self.init_cid();
    }

    fn parse_private_dict(&mut self, data: &Vec<u8>) {
        let mut i = 0;
        let mut temp = Vec::new();
        let mut private = Private::default();

        macro_rules! _pop_num { () => { temp.pop().unwrap() }; }
        macro_rules! _pop_bool { () => { _pop_num!().integer() != 0 }; }
        macro_rules! _pop_array {
            () => ({
                let nums_copy = temp.to_vec();
                temp.clear();
                nums_copy
            });
        }
        macro_rules! _pop_delta { () => { Delta::new(_pop_array!()) }; }

        while i < data.len() {
            let b0 = data[i];
            match b0 {
                6 => private.blue_values = _pop_delta!(),
                7 => private.other_blues = _pop_delta!(),
                8 => private.family_blues = _pop_delta!(),
                9 => private.family_other_blues = _pop_delta!(),
                10 => private.std_hw =_pop_num!(),
                11 => private.std_vw =_pop_num!(),
                12 => {
                    let b1 = data[i + 1];
                    match b1 {
                        9 => private.blue_scale =_pop_num!(),
                        10 => private.blue_shift =_pop_num!(),
                        11 => private.blue_fuzz =_pop_num!(),
                        12 => private.stem_snap_h = _pop_delta!(),
                        13 => private.stem_snap_v = _pop_delta!(),
                        14 => private.force_bold = _pop_bool!(),
                        17 => private.language_group =_pop_num!(),
                        18 => private.expansion_factor =_pop_num!(),
                        19 => private.initial_random_seed =_pop_num!(),
                        _ => unreachable!(),
                    }
                    i += 1;
                }
                19 => private.subrs = Some(_pop_num!()),
                20 => private.default_width_x =_pop_num!(),
                21 => private.nominal_width_x =_pop_num!(),
                // Operands
                30 => temp.push(Self::get_real(&data, &mut i)),
                _ => temp.push(Self::get_integer(&data, &mut i, b0)),
            }
            i += 1;
        }

        self.private = Some(private);
    }

    fn init_cid(&mut self) {
        if let Some(_) = self.ros {
            macro_rules! _init_cid {
                ($i:ident, $e:expr) => {
                    if self.$i.is_none() { self.$i = Some($e); }
                };
            }
            _init_cid!(cid_font_version, Number::Integer(0));
            _init_cid!(cid_font_revision, Number::Integer(0));
            _init_cid!(cid_font_type, 0);
            _init_cid!(cid_count, 8720);
        }
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
            _ => unreachable!(),
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
            _encoding_offset: 0,
            encoding: Default::default(),
            _charset_offset: 0,
            charset: Default::default(),
            _char_strings_offset: Default::default(),
            char_strings: Default::default(),
            _private_size: Default::default(),
            _private_offset: Default::default(),
            private: Default::default(),
            ros: Default::default(),
            cid_font_version: Default::default(),
            cid_font_revision: Default::default(),
            cid_font_type: Default::default(),
            cid_count: Default::default(),
            uid_base: Default::default(),
            _fd_array_offset: Default::default(),
            _fd_select_offset: Default::default(),
            cid_font_name: Default::default(),
            global_subr: Default::default(),
        }
    }
}

#[derive(Debug)]
enum Encoding {
    Standard,
    Expert,
    Custom {
        format: u8,
        // Format 0
        num_codes: Option<u8>,
        code: Option<Vec<u8>>,
        // Format 1
        num_ranges: Option<u8>,
        range: Option<Vec<EncodingRange>>,
    }
}

impl Default for Encoding {
    fn default() -> Self {
        Self::Standard
    }
}

impl ReadBuffer for Encoding {
    fn read(buffer: &mut Buffer) -> Self {
        let format = buffer.get();
        let mut num_codes = None;
        let mut code = None;
        let mut num_ranges = None;
        let mut range = None;
        match format {
            0 => {
                num_codes = Some(buffer.get());
                code = Some(buffer.get_vec(num_codes.unwrap() as usize));
            }
            1 => {
                num_ranges = Some(buffer.get());
                range = Some(buffer.get_vec(num_ranges.unwrap() as usize));
            }
            _ => unreachable!(),
        }
        Self::Custom {
            format,
            num_codes,
            code,
            num_ranges,
            range,
        }
    }
}

#[derive(Debug)]
struct EncodingRange {
    first: u8,
    num_left: u8,
}

impl ReadBuffer for EncodingRange {
    fn read(buffer: &mut Buffer) -> Self {
        Self {
            first: buffer.get(),
            num_left: buffer.get(),
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

#[derive(Debug)]
struct ROS {
    registry: String,
    ordering: String,
    supplement: i32,
}

impl ROS {
    fn new(index_r: usize, index_o: usize, supplement: i32, strings: &Vec<String>) -> Self {
        Self {
            registry: from_sid(index_r, strings),
            ordering: from_sid(index_o, strings),
            supplement,
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
    let len = CFF_STANDARD_STRINGS.len();
    if sid < len {
        CFF_STANDARD_STRINGS[sid].to_string()
    } else {
        strings[sid - len].to_string()
    }
}
