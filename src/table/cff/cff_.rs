use std::fmt;

use crate::font::Font;
use crate::table::cff::cff_data::*;
use crate::util::{u24, Buffer, ReadBuffer};
use read_buffer_derive::ReadBuffer;

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
    cff_fonts: Vec<CFFFont>,
    global_subrs: Vec<Subr>,
}

impl Font {
    #[allow(non_snake_case)]
    pub fn parse_CFF_(&mut self, buffer: &mut Buffer) {
        let cff_start_offset = buffer.offset;
        let _version = buffer.get_version::<u8>();
        let _header_size = buffer.get();
        let _offset_size = buffer.get();
        buffer.offset = cff_start_offset + _header_size as usize;

        let names = buffer.get::<Index>().to_string_vec();
        let top_dicts = buffer.get::<Index>().data;
        let strings = buffer.get::<Index>().to_string_vec();
        let global_subrs = buffer.get::<Index>().data;

        self.CFF_ = Some(Table_CFF_ {
            _version,
            _header_size,
            _offset_size,
            cff_fonts: names
                .into_iter()
                .zip(top_dicts.iter())
                .map(|(name, top_dict)| {
                    let mut cff = CFFFont::new(name);
                    cff.parse(buffer, cff_start_offset, top_dict, &strings);
                    // FIXME: debug code
                    // cff.char_strings = Default::default();
                    // cff.charset = Default::default();
                    cff
                })
                .collect(),
            global_subrs,
        });
    }
}

macro_rules! _parse_dict {
    (
        $data:expr;
        $strings:expr;
        [
            $e00:expr,
            $e01:expr,
            $e02:expr,
            $e03:expr,
            $e04:expr,
            $e05:expr,
            $e06:expr,
            $e07:expr,
            $e08:expr,
            $e09:expr,
            $e10:expr,
            $e11:expr,
            $e12_00:expr,
            $e12_01:expr,
            $e12_02:expr,
            $e12_03:expr,
            $e12_04:expr,
            $e12_05:expr,
            $e12_06:expr,
            $e12_07:expr,
            $e12_08:expr,
            $e12_09:expr,
            $e12_10:expr,
            $e12_11:expr,
            $e12_12:expr,
            $e12_13:expr,
            $e12_14:expr,
            $e12_17:expr,
            $e12_18:expr,
            $e12_19:expr,
            $e12_20:expr,
            $e12_21:expr,
            $e12_22:expr,
            $e12_23:expr,
            $e12_30:expr,
            $e12_31:expr,
            $e12_32:expr,
            $e12_33:expr,
            $e12_34:expr,
            $e12_35:expr,
            $e12_36:expr,
            $e12_37:expr,
            $e12_38:expr,
            $e13:expr,
            $e14:expr,
            $e15:expr,
            $e16:expr,
            $e17:expr,
            $e18:expr,
            $e19:expr,
            $e20:expr,
            $e21:expr,
        ]
    ) => {{
        let mut i = 0;
        let mut temp: Vec<Number> = Vec::new();

        macro_rules! _num {
            () => {
                temp.pop().unwrap()
            };
        }
        macro_rules! _integer {
            () => {
                _num!().int()
            };
        }
        macro_rules! _bool {
            () => {
                _integer!() != 0
            };
        }
        macro_rules! _string {
            () => {
                from_sid(_integer!() as usize, $strings)
            };
        }
        macro_rules! _array {
            () => {{
                let nums_copy = temp.to_vec();
                temp.clear();
                nums_copy
            }};
        }
        macro_rules! _delta {
            () => {
                Delta::new(_array!())
            };
        }
        macro_rules! _private {
            () => {{
                let num2 = temp.pop().unwrap().int() as usize;
                let num1 = temp.pop().unwrap().int() as usize;
                (num1, num2)
            }};
        }
        macro_rules! _ros {
            () => {{
                let supplement = temp.pop().unwrap().int();
                let index_o = temp.pop().unwrap().int() as usize;
                let index_r = temp.pop().unwrap().int() as usize;
                ROS::new(index_r, index_o, supplement, $strings)
            }};
        }

        while i < $data.len() {
            let b0 = $data[i];
            match b0 {
                // Operators: one byte
                0 => $e00,
                1 => $e01,
                2 => $e02,
                3 => $e03,
                4 => $e04,
                5 => $e05,
                6 => $e06,
                7 => $e07,
                8 => $e08,
                9 => $e09,
                10 => $e10,
                11 => $e11,
                13 => $e13,
                14 => $e14,
                15 => $e15,
                16 => $e16,
                17 => $e17,
                18 => $e18,
                19 => $e19,
                20 => $e20,
                21 => $e21,
                // Operators: two bytes
                12 => {
                    let b1 = $data[i + 1];
                    match b1 {
                        0 => $e12_00,
                        1 => $e12_01,
                        2 => $e12_02,
                        3 => $e12_03,
                        4 => $e12_04,
                        5 => $e12_05,
                        6 => $e12_06,
                        7 => $e12_07,
                        8 => $e12_08,
                        9 => $e12_09,
                        10 => $e12_10,
                        11 => $e12_11,
                        12 => $e12_12,
                        13 => $e12_13,
                        14 => $e12_14,
                        17 => $e12_17,
                        18 => $e12_18,
                        19 => $e12_19,
                        20 => $e12_20,
                        21 => $e12_21,
                        22 => $e12_22,
                        23 => $e12_23,
                        30 => $e12_30,
                        31 => $e12_31,
                        32 => $e12_32,
                        33 => $e12_33,
                        34 => $e12_34,
                        35 => $e12_35,
                        36 => $e12_36,
                        37 => $e12_37,
                        38 => $e12_38,
                        _ => unreachable!(),
                    }
                    i += 1;
                }
                // Operands: integers
                32..=246 => {
                    let b0 = b0 as i32;
                    temp.push(Number::Int(b0 - 139));
                }
                247..=250 => {
                    let b0 = b0 as i32;
                    let b1 = $data[i + 1] as i32;
                    i += 1;
                    temp.push(Number::Int((b0 - 247) * 256 + b1 + 108));
                }
                251..=254 => {
                    let b0 = b0 as i32;
                    let b1 = $data[i + 1] as i32;
                    i += 1;
                    temp.push(Number::Int(-(b0 - 251) * 256 - b1 - 108));
                }
                28 => {
                    let b1 = $data[i + 1] as i16;
                    let b2 = $data[i + 2] as i16;
                    i += 2;
                    temp.push(Number::Int((b1 << 8 | b2) as i32));
                }
                29 => {
                    let b1 = $data[i + 1] as i32;
                    let b2 = $data[i + 2] as i32;
                    let b3 = $data[i + 3] as i32;
                    let b4 = $data[i + 4] as i32;
                    i += 4;
                    temp.push(Number::Int(b1 << 24 | b2 << 16 | b3 << 8 | b4));
                }
                // Operands: reals
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
                        let b1 = $data[i + 1];
                        _match_nibble!(b1 >> 4);
                        _match_nibble!((b1 << 4) >> 4);
                        i += 1;
                    }
                }
                _ => unreachable!(),
            }
            i += 1;
        }
    }};
}

#[derive(Debug, Default)]
pub struct CFFFont {
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
    fd_array: Vec<FDArray>,
    _fd_select_offset: Option<usize>,
    fd_select: Option<FDSelect>,
    cid_font_name: Option<String>,
}

impl CFFFont {
    fn new(name: String) -> Self {
        Self {
            name,
            is_fixed_pitch: false,
            italic_angle: Number::Int(0),
            underline_position: Number::Int(-100),
            underline_thickness: Number::Int(50),
            paint_type: 0,
            char_string_type: 2,
            font_matrix: vec![
                Number::Real((0.001).to_string()),
                Number::Real((0.0).to_string()),
                Number::Real((0.001).to_string()),
                Number::Real((0.0).to_string()),
            ],
            font_bbox: vec![
                Number::Int(0),
                Number::Int(0),
                Number::Int(0),
                Number::Int(0),
            ],
            stroke_width: Number::Int(0),
            _encoding_offset: 0,
            _charset_offset: 0,
            ..Default::default()
        }
    }

    fn parse(
        &mut self,
        buffer: &mut Buffer,
        cff_start_offset: usize,
        top_dict: &Vec<u8>,
        strings: &Vec<String>,
    ) {
        self.parse_top_dict(top_dict, strings);
        // Encoding
        self.encoding = match self._encoding_offset {
            0 => Encoding::Standard,
            1 => Encoding::Expert,
            _ => {
                buffer.offset = cff_start_offset + self._encoding_offset;
                buffer.get()
            }
        };
        // Char strings
        buffer.offset = cff_start_offset + self._char_strings_offset;
        let char_strings_index = buffer.get::<Index>();
        let num_glyphs = char_strings_index.count;
        self.char_strings = char_strings_index.data;
        // Charset
        macro_rules! _get_charsets {
            ($t:ty) => {{
                // ".notdef" is omitted in the array.
                let mut count = 1;
                let mut result = vec![CFF_STANDARD_STRINGS[0].to_string()];
                while count < num_glyphs {
                    let sid = buffer.get::<u16>() as usize;
                    let num_left = buffer.get::<$t>() as usize;
                    (0..=num_left).for_each(|i| result.push(from_sid(sid + i, &strings)));
                    count += num_left as usize + 1;
                }
                result
            }};
        }
        if !self.is_cid_font() {
            self.charset = match self._charset_offset {
                0 => CFF_ISO_ADOBE_CHARSET
                    .iter()
                    .map(|&i| i.to_string())
                    .collect(),
                1 => CFF_EXPERT_CHARSET.iter().map(|&i| i.to_string()).collect(),
                2 => CFF_EXPERT_SUBSET_CHARSET
                    .iter()
                    .map(|&i| i.to_string())
                    .collect(),
                offset => {
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
            // Private dict
            if self._private_size != 0 {
                let private_start_offset = cff_start_offset + self._private_offset;
                buffer.offset = private_start_offset;
                let private_dict = buffer.get_vec(self._private_size);
                let mut private = Private::parse(&private_dict);
                if let Some(subrs_offset) = private._subrs_offset {
                    buffer.offset = private_start_offset + subrs_offset;
                    private.subrs = buffer.get::<Index>().data;
                }
                self.private = Some(private);
            }
        } else {
            // FD Array
            buffer.offset = cff_start_offset + self._fd_array_offset.unwrap();
            buffer
                .get::<Index>()
                .data
                .iter()
                .for_each(|font_dict| self.init_fd_array(font_dict, strings));
            self.fd_array.iter_mut().for_each(|fd| {
                let private_start_offset = cff_start_offset + fd._private_offset;
                buffer.offset = private_start_offset;
                let private_dict = buffer.get_vec(fd._private_size);
                fd.private = Private::parse(&private_dict);
                if let Some(subrs_offset) = fd.private._subrs_offset {
                    buffer.offset = private_start_offset + subrs_offset;
                    fd.private.subrs = buffer.get::<Index>().data;
                }
            });
            // FD Select
            buffer.offset = cff_start_offset + self._fd_select_offset.unwrap();
            self.fd_select = Some(FDSelect::read(buffer, num_glyphs));
        }
    }

    fn parse_top_dict(&mut self, top_dict: &Vec<u8>, strings: &Vec<String>) {
        _parse_dict!(top_dict; strings; [
            /* 00    */ self.version = _string!(),
            /* 01    */ self.notice = _string!(),
            /* 02    */ self.full_name = _string!(),
            /* 03    */ self.family_name = _string!(),
            /* 04    */ self.weight = _string!(),
            /* 05    */ self.font_bbox = _array!(),
                        {}, {}, {}, {}, {}, {},
            /* 12 00 */ self.copyright = _string!(),
            /* 12 01 */ self.is_fixed_pitch = _bool!(),
            /* 12 02 */ self.italic_angle = _num!(),
            /* 12 03 */ self.underline_position = _num!(),
            /* 12 04 */ self.underline_thickness = _num!(),
            /* 12 05 */ self.paint_type = _integer!(),
            /* 12 06 */ self.char_string_type = _integer!(),
            /* 12 07 */ self.font_matrix = _array!(),
            /* 12 08 */ self.stroke_width = _num!(),
                        {}, {}, {}, {}, {}, {}, {}, {}, {},
            /* 12 20 */ self.synthetic_base = Some(_integer!()),
            /* 12 21 */ self.postscript = Some(_string!()),
            /* 12 22 */ self.base_font_name = Some(_string!()),
            /* 12 23 */ self.base_font_blend = Some(_delta!()),
            /* 12 30 */ self.ros = Some(_ros!()),
            /* 12 31 */ self.cid_font_version = Some(_num!()),
            /* 12 32 */ self.cid_font_revision = Some(_num!()),
            /* 12 33 */ self.cid_font_type = Some(_integer!()),
            /* 12 34 */ self.cid_count = Some(_integer!()),
            /* 12 35 */ self.uid_base = Some(_integer!()),
            /* 12 36 */ self._fd_array_offset = Some(_integer!() as usize),
            /* 12 37 */ self._fd_select_offset = Some(_integer!() as usize),
            /* 12 38 */ self.cid_font_name = Some(_string!()),
            /* 13    */ self.unique_id = Some(_integer!()),
            /* 14    */ self.xuid = Some(_array!()),
            /* 15    */ self._charset_offset = _integer!() as usize,
            /* 16    */ self._encoding_offset = _integer!() as usize,
            /* 17    */ self._char_strings_offset = _integer!() as usize,
            /* 18    */ {
                           let private = _private!();
                           self._private_size = private.0;
                           self._private_offset = private.1;
                        },
                        {}, {}, {},
        ]);
        self.init_cid();
    }

    fn init_fd_array(&mut self, font_dict: &Vec<u8>, strings: &Vec<String>) {
        let mut font_name = Default::default();
        let mut _private_size = 0;
        let mut _private_offset = 0;
        _parse_dict!(font_dict; strings; [
                        {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {},
                        {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {},
                        {}, {}, {}, {}, {}, {}, {}, {}, {}, {},
            /* 12 38 */ font_name = _string!(),
                        {}, {}, {}, {}, {},
            /* 18    */ {
                           let private = _private!();
                           _private_size = private.0;
                           _private_offset = private.1;
                        },
                        {}, {}, {},
        ]);
        self.fd_array.push(FDArray {
            font_name,
            _private_size,
            _private_offset,
            ..Default::default()
        });
    }

    fn init_cid(&mut self) {
        if self.is_cid_font() {
            macro_rules! _init_cid {
                ($i:ident, $e:expr) => {
                    if self.$i.is_none() {
                        self.$i = Some($e);
                    }
                };
            }
            _init_cid!(cid_font_version, Number::Int(0));
            _init_cid!(cid_font_revision, Number::Int(0));
            _init_cid!(cid_font_type, 0);
            _init_cid!(cid_count, 8720);
        }
    }

    const fn is_cid_font(&self) -> bool {
        self.ros.is_some()
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
    },
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

#[derive(Debug, ReadBuffer)]
struct EncodingRange {
    first: u8,
    num_left: u8,
}

#[derive(Debug, Default)]
struct Private {
    _size: usize,
    _offset: usize,
    blue_values: Option<Delta>,
    other_blues: Option<Delta>,
    family_blues: Option<Delta>,
    family_other_blues: Option<Delta>,
    blue_scale: Number,
    blue_shift: Number,
    blue_fuzz: Number,
    std_hw: Option<Number>,
    std_vw: Option<Number>,
    stem_snap_h: Option<Delta>,
    stem_snap_v: Option<Delta>,
    force_bold: bool,
    language_group: Number,
    expansion_factor: Number,
    initial_random_seed: Number,
    _subrs_offset: Option<usize>,
    subrs: Vec<Subr>,
    default_width_x: Number,
    nominal_width_x: Number,
}

impl Private {
    fn new() -> Self {
        Self {
            blue_scale: Number::Real((0.039625).to_string()),
            blue_shift: Number::Int(7),
            blue_fuzz: Number::Int(1),
            force_bold: false,
            language_group: Number::Int(0),
            expansion_factor: Number::Real((0.06).to_string()),
            initial_random_seed: Number::Int(0),
            default_width_x: Number::Int(0),
            nominal_width_x: Number::Int(0),
            ..Default::default()
        }
    }

    fn parse(private_dict: &Vec<u8>) -> Self {
        let mut private = Self::new();
        let _strings: Vec<String> = Vec::new(); // A placeholder to make the macro work
        _parse_dict!(private_dict; &_strings; [
                        {}, {}, {}, {}, {}, {},
            /* 06    */ private.blue_values = Some(_delta!()),
            /* 07    */ private.other_blues = Some(_delta!()),
            /* 08    */ private.family_blues = Some(_delta!()),
            /* 09    */ private.family_other_blues = Some(_delta!()),
            /* 10    */ private.std_hw = Some(_num!()),
            /* 11    */ private.std_vw = Some(_num!()),
                        {}, {}, {}, {}, {}, {}, {}, {}, {},
            /* 12 09 */ private.blue_scale = _num!(),
            /* 12 10 */ private.blue_shift = _num!(),
            /* 12 11 */ private.blue_fuzz = _num!(),
            /* 12 12 */ private.stem_snap_h = Some(_delta!()),
            /* 12 13 */ private.stem_snap_v = Some(_delta!()),
            /* 12 14 */ private.force_bold = _bool!(),
            /* 12 17 */ private.language_group = _num!(),
            /* 12 18 */ private.expansion_factor = _num!(),
            /* 12 19 */ private.initial_random_seed = _num!(),
                        {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {},
            /* 19    */ private._subrs_offset = Some(_integer!() as usize),
            /* 20    */ private.default_width_x = _num!(),
            /* 21    */ private.nominal_width_x = _num!(),
        ]);
        private
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

#[derive(Debug, Default)]
struct FDArray {
    font_name: String,
    _private_size: usize,
    _private_offset: usize,
    private: Private,
}

#[derive(Debug, Default)]
struct FDSelect {
    format: u8,
    // Format 0
    fd_selector_array: Vec<u8>,
    // Format 3
    num_ranges: Option<u16>,
    range: Vec<FDSelectRange>,
    sentinel: Option<u16>,
}

impl FDSelect {
    fn read(buffer: &mut Buffer, num_glyphs: usize) -> Self {
        let format = buffer.get();
        let mut fd_select = Self {
            format,
            ..Default::default()
        };
        match format {
            0 => {
                fd_select.fd_selector_array = buffer.get_vec(num_glyphs);
            }
            3 => {
                fd_select.num_ranges = Some(buffer.get());
                fd_select.range = buffer.get_vec(fd_select.num_ranges.unwrap() as usize);
                fd_select.sentinel = Some(buffer.get());
            }
            _ => unreachable!(),
        }
        fd_select
    }
}

#[derive(Debug, Default, ReadBuffer)]
struct FDSelectRange {
    first: u16,
    fd: u8,
}

type Subr = Vec<u8>;

#[derive(Clone)]
enum Number {
    Int(i32),
    Real(String),
}

impl Number {
    fn int(self) -> i32 {
        if let Self::Int(n) = self {
            n
        } else {
            panic!()
        }
    }
}

impl fmt::Debug for Number {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Int(n) => write!(f, "{}", n),
            Self::Real(n) => write!(f, "{}", n),
        }
    }
}

impl Default for Number {
    fn default() -> Self {
        Number::Int(0)
    }
}

#[derive(Default)]
struct Delta {
    _data: Vec<Number>,
}

impl Delta {
    fn new(array: Vec<Number>) -> Self {
        Self {
            _data: array
                .iter()
                .scan(0, |acc, x| {
                    *acc = *acc + x.to_owned().int();
                    Some(Number::Int(*acc))
                })
                .collect(),
        }
    }
}

impl fmt::Debug for Delta {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self._data)
    }
}

/// An array of variable-sized objects.
#[derive(Debug, Default)]
struct Index {
    count: usize, // Actual type is `u16`
    offset_size: u8,
    offset: Vec<usize>, // Actual type is `Offset[]`
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
        match count {
            0 => Default::default(),
            _ => {
                macro_rules! _get_offset {
                    (u24) => {
                        buffer
                            .get_vec::<u24>(count + 1)
                            .iter()
                            .map(|&i| usize::from(i))
                            .collect()
                    };
                    ($t:ty) => {
                        buffer
                            .get_vec::<$t>(count + 1)
                            .iter()
                            .map(|&i| i as usize)
                            .collect()
                    };
                }
                let offset_size = buffer.get();
                let offset: Vec<usize> = match offset_size {
                    1 => _get_offset!(u8),
                    2 => _get_offset!(u16),
                    3 => _get_offset!(u24),
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
}

fn from_sid(sid: usize, strings: &Vec<String>) -> String {
    let len = CFF_STANDARD_STRINGS.len();
    if sid < len {
        CFF_STANDARD_STRINGS[sid].to_string()
    } else {
        strings[sid - len].to_string()
    }
}
