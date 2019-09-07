// extern crate byteorder;

use std::{fmt, mem};

use byteorder::{BigEndian, ByteOrder};
use chrono::NaiveDateTime;

pub fn get_version_string(major: u16, minor: u16) -> String {
    major.to_string() + "." + &minor.to_string()
}

pub struct Buffer {
    _buffer: Vec<u8>,
    pub offset: usize,
}

impl Buffer {
    /// Create a new `Buffer`.
    pub const fn new(buffer: Vec<u8>) -> Buffer {
        Buffer {
            _buffer: buffer,
            offset: 0,
        }
    }

    /// Get a value as type `T` from the buffer.
    pub fn get<T: Read>(&mut self) -> T {
        // let _offset = self.offset as usize;
        // self.offset += mem::size_of::<T>() as u32;
        Read::read(self)
    }

    pub fn get_vec<T: Read>(&mut self, n: usize) -> Vec<T> {
        // let mut _offset = self.offset as usize;
        // let _size = mem::size_of::<T>();
        // let mut v: Vec<T> = Vec::new();
        // for _ in 0..n {
        //     let elem = Read::read(&self._buffer, _offset);
        //     _offset += _size;
        //     v.push(elem);
        // }
        // self.offset = _offset as u32;
        // v
        (0..n).map(|_| Read::read(self)).collect()
    }

    /// Skip `n` * `size_of<T>` bytes for `offset`.
    pub fn skip<T>(&mut self, n: usize) {
        self.offset += n * mem::size_of::<T>();
    }

    pub fn slice(&mut self, start: usize, end: usize) -> &[u8] {
        &self._buffer[self.offset + start..self.offset + end]
    }

    // pub fn calc_check_sum(&self, offset: u32, length: u32) -> u32 {
    //     let _offset = offset as usize;
    //     let padded_length = ((length + 3) & !3) as usize;
    //     (0..padded_length).step_by(4).fold(0, |acc, i| {
    //         acc.wrapping_add(BigEndian::read_u32(
    //             &self.buffer[_offset + i.._offset + i + 4],
    //         ))
    //     })
    // }
}

pub trait Read {
    // fn read(_buffer: &Vec<u8>, _offset: usize) -> Self;
    fn read(buffer: &mut Buffer) -> Self;
}

/// The following data types are used in the OpenType font file. All OpenType
/// fonts use Motorola-style byte ordering (Big Endian):
///
/// - `uint8`
/// - `int8`
/// - `uint16`
/// - `int16`
/// - `uint24`
/// - `uint32`
/// - `int32`
/// - `Fixed`
/// - `FWord`
/// - `UFWord`
/// - `F2Dot14`
/// - `LongDateTime`
/// - `Tag`
/// - `Offset16`
/// - `Offset32`

impl Read for u8 {
    fn read(buffer: &mut Buffer) -> Self {
        let offset = buffer.offset;
        buffer.offset += mem::size_of::<u8>();
        buffer._buffer[offset]
    }
}

impl Read for i8 {
    fn read(buffer: &mut Buffer) -> Self {
        let offset = buffer.offset;
        buffer.offset += mem::size_of::<i8>();
        buffer._buffer[offset] as i8
    }
}

// Implement `Read` for `u16`, `u32`, etc.
macro_rules! _generate_read {
    ($t:ty, $f:expr) => {
        impl Read for $t {
            fn read(buffer: &mut Buffer) -> Self {
                let offset = buffer.offset;
                buffer.offset += mem::size_of::<$t>();
                $f(&buffer._buffer[offset..buffer.offset])
            }
        }
    };
}

_generate_read!(u16, BigEndian::read_u16);
_generate_read!(u32, BigEndian::read_u32);
_generate_read!(u64, BigEndian::read_u64);
_generate_read!(i16, BigEndian::read_i16);
_generate_read!(i32, BigEndian::read_i32);
_generate_read!(i64, BigEndian::read_i64);

#[allow(non_camel_case_types)]
pub struct u24 {
    _internal: [u8; 3],
}

impl fmt::Debug for u24 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let num =
            4 * self._internal[0] as u32 + 2 * self._internal[1] as u32 + self._internal[2] as u32;
        write!(f, "{}", num)
    }
}

impl Read for u24 {
    fn read(buffer: &mut Buffer) -> Self {
        Self {
            _internal: [buffer.get::<u8>(), buffer.get::<u8>(), buffer.get::<u8>()],
        }
    }
}

/// 32-bit signed fixed-point number (16.16).
pub struct Fixed {
    _num: i32,
}

impl fmt::Debug for Fixed {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:.3}", self._num as f64 / 65536.0)
    }
}

impl PartialEq<i32> for Fixed {
    fn eq(&self, other: &i32) -> bool {
        self._num == *other
    }
}

impl Read for Fixed {
    fn read(buffer: &mut Buffer) -> Self {
        Self {
            _num: buffer.get::<i32>(),
        }
    }
}

/// 16-bit signed fixed number with the low 14 bits of fraction (2.14).
// pub struct F2Dot14 {
//     _num: i16,
// }

// impl fmt::Debug for F2Dot14 {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         write!(f, "{:.3}", self._num as f64 / 16384.0)
//     }
// }

// impl PartialEq<i16> for F2Dot14 {
//     fn eq(&self, other: &i16) -> bool {
//         self._num == *other
//     }
// }

// impl Read for F2Dot14 {
//     fn read(_buffer: &Vec<u8>, _offset: usize) -> Self {
//         Self {
//             _num: i16::read(_buffer, _offset),
//         }
//     }
// }

/// Date represented in number of seconds since 12:00 midnight, January 1, 1904.
/// The value is represented as a signed 64-bit integer.
pub struct LongDateTime {
    _num: i64,
}

/// Seconds from 1904-01-01 to 1970-01-01 (at midnight).
const DATE_TIME_OFFSET: i64 = 2082844800;

impl fmt::Debug for LongDateTime {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let timestamp = self._num - DATE_TIME_OFFSET;
        write!(f, "{}", NaiveDateTime::from_timestamp(timestamp, 0))
    }
}

impl Read for LongDateTime {
    fn read(buffer: &mut Buffer) -> Self {
        Self {
            _num: buffer.get::<i64>(),
        }
    }
}

/// Array of four `uint8`s (length = 32 bits) used to identify a table,
/// design-variation axis, script, language system, feature, or baseline.
///
/// **Note:** In Rust, `char` is a *Unicode scalar value* with a size of 4 bytes
/// rather than 1, so it can't be used here.
pub struct Tag {
    _u8_arr: [u8; 4],
}

impl Tag {
    pub fn to_string(&self) -> String {
        let char_arr = [
            self._u8_arr[0] as char,
            self._u8_arr[1] as char,
            self._u8_arr[2] as char,
            self._u8_arr[3] as char,
        ];
        char_arr.iter().collect()
    }
}

impl fmt::Debug for Tag {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

impl Read for Tag {
    fn read(buffer: &mut Buffer) -> Self {
        Self {
            _u8_arr: [
                buffer.get::<u8>(),
                buffer.get::<u8>(),
                buffer.get::<u8>(),
                buffer.get::<u8>(),
            ],
        }
    }
}

pub type FWord = i16;
pub type UFWord = u16;
pub type Offset16 = u16;
pub type Offset32 = u32;

// pub const I8_SIZE: usize = mem::size_of::<i8>();
// pub const I16_SIZE: usize = mem::size_of::<i16>();
// pub const I32_SIZE: usize = mem::size_of::<i32>();
// pub const U8_SIZE: usize = mem::size_of::<u8>();
// pub const U16_SIZE: usize = mem::size_of::<u16>();
// pub const U32_SIZE: usize = mem::size_of::<u32>();
// pub const FIXED_SIZE: usize = mem::size_of::<Fixed>();
// pub const FWORD_SIZE: usize = mem::size_of::<FWord>();
// pub const UFWORD_SIZE: usize = mem::size_of::<UFWord>();
// pub const F2DOT14_SIZE: usize = mem::size_of::<F2Dot14>();
// pub const LONG_DATE_TIME_SIZE: usize = mem::size_of::<LongDateTime>();
// pub const TAG_SIZE: usize = mem::size_of::<Tag>();
// pub const OFFSET16_SIZE: usize = mem::size_of::<Offset16>();
// pub const OFFSET32_SIZE: usize = mem::size_of::<Offset32>();
