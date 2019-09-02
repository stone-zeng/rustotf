// extern crate byteorder;

use std::fmt;
use std::mem;

use byteorder::{BigEndian, ByteOrder};
use chrono::NaiveDateTime;

pub fn get_version_string(major: u16, minor: u16) -> String {
    major.to_string() + "." + &minor.to_string()
}

pub struct Buffer {
    _buffer: Vec<u8>,
    pub offset: u32,
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
    pub fn read<T: ReadFromBuffer>(&mut self) -> T {
        let _offset = self.offset as usize;
        self.offset += mem::size_of::<T>() as u32;
        ReadFromBuffer::read_from_buffer(&self._buffer, _offset)
    }

    pub fn read_vec<T: ReadFromBuffer>(&mut self, n: u32) -> Vec<T> {
        let mut _offset = self.offset as usize;
        let _size = mem::size_of::<T>();
        let mut v: Vec<T> = Vec::new();
        for _ in 0..n {
            let elem = ReadFromBuffer::read_from_buffer(&self._buffer, _offset);
            _offset += _size;
            v.push(elem);
        }
        self.offset = _offset as u32;
        v
    }

    // pub fn read_arr<T: ReadFromBuffer>(&mut self, n: u32) -> [T; n] {
    //     let mut _offset = self.offset as usize;
    //     let _size = mem::size_of::<T>();
    //     let mut v: Vec<T> = Vec::new();
    //     for _ in 0..n {
    //         let elem = ReadFromBuffer::read_from_buffer(&self._buffer, _offset);
    //         _offset += _size;
    //         v.push(elem);
    //     }
    //     self.offset = _offset as u32;
    //     v
    // }

    /// Skip `n` * `size_of<T>` bytes for `offset`.
    pub fn skip<T>(&mut self, n: u32) {
        self.offset += n * mem::size_of::<T>() as u32;
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

pub trait ReadFromBuffer {
    fn read_from_buffer(_buffer: &Vec<u8>, _offset: usize) -> Self;
}

impl ReadFromBuffer for u8 {
    fn read_from_buffer(_buffer: &Vec<u8>, _offset: usize) -> Self {
        _buffer[_offset]
    }
}

// Implement `ReadFromBuffer` for `u16`, `u32`, etc.
macro_rules! _generate_read_from_buffer {
    ($t:ty, $f:expr) => {
        impl ReadFromBuffer for $t {
            fn read_from_buffer(_buffer: &Vec<u8>, _offset: usize) -> Self {
                $f(&_buffer[_offset.._offset + mem::size_of::<$t>()])
            }
        }
    };
}

_generate_read_from_buffer!(u16, BigEndian::read_u16);
_generate_read_from_buffer!(u32, BigEndian::read_u32);
_generate_read_from_buffer!(u64, BigEndian::read_u64);
_generate_read_from_buffer!(i16, BigEndian::read_i16);
_generate_read_from_buffer!(i32, BigEndian::read_i32);
_generate_read_from_buffer!(i64, BigEndian::read_i64);

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

impl ReadFromBuffer for Fixed {
    fn read_from_buffer(_buffer: &Vec<u8>, _offset: usize) -> Self {
        Self {
            _num: i32::read_from_buffer(_buffer, _offset),
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

// impl ReadFromBuffer for F2Dot14 {
//     fn read_from_buffer(_buffer: &Vec<u8>, _offset: usize) -> Self {
//         Self {
//             _num: i16::read_from_buffer(_buffer, _offset),
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

impl ReadFromBuffer for LongDateTime {
    fn read_from_buffer(_buffer: &Vec<u8>, _offset: usize) -> Self {
        Self {
            _num: i64::read_from_buffer(_buffer, _offset),
        }
    }
}

/// Array of four `uint8`s (length = 32 bits) used to identify a table,
/// design-variation axis, script, language system, feature, or baseline.
///
/// Note: In Rust, `char` is a *Unicode scalar value* with a size of 4 bytes
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

impl ReadFromBuffer for Tag {
    fn read_from_buffer(_buffer: &Vec<u8>, _offset: usize) -> Self {
        Self {
            _u8_arr: [
                u8::read_from_buffer(_buffer, _offset),
                u8::read_from_buffer(_buffer, _offset + 1),
                u8::read_from_buffer(_buffer, _offset + 2),
                u8::read_from_buffer(_buffer, _offset + 3),
            ],
        }
    }
}
