use std::{fmt, io::Read, mem, str};

use byteorder::{BigEndian, ByteOrder};
use chrono::NaiveDateTime;
use flate2::read::{GzDecoder, ZlibDecoder};
use read_buffer_derive::ReadBuffer;

pub struct Buffer {
    raw_buffer: Vec<u8>,
    pub offset: usize,
}

impl Buffer {
    /// Create a new `Buffer`.
    pub fn new(raw_buffer: Vec<u8>) -> Self {
        Self {
            raw_buffer,
            offset: 0,
        }
    }

    /// Return the length of the buffer.
    pub fn len(&self) -> usize {
        self.raw_buffer.len()
    }

    /// Get a value as type `T` from the buffer.
    pub fn get<T: ReadBuffer>(&mut self) -> T {
        ReadBuffer::read(self)
    }

    /// Get a vector of type `T` values from the buffer.
    pub fn get_vec<T: ReadBuffer>(&mut self, n: usize) -> Vec<T> {
        (0..n).map(|_| ReadBuffer::read(self)).collect()
    }

    /// Get an option of type `T` values from the buffer.
    /// If `offset` is 0 (i.e. NULL), then it will return a `None`.
    pub fn get_or_none<T: ReadBuffer>(&mut self, start_offset: usize, offset: usize) -> Option<T> {
        if offset != 0 {
            self.offset = start_offset + offset;
            Some(self.get::<T>())
        } else {
            None
        }
    }

    /// Get a version string (`major.minor`) from the buffer.
    pub fn get_version<T: ReadBuffer + fmt::Display>(&mut self) -> String {
        format!("{}.{}", self.get::<T>(), self.get::<T>())
    }

    /// Skip `n` * `size_of<T>` bytes for `offset`.
    pub fn skip<T>(&mut self, n: usize) {
        self.offset += n * mem::size_of::<T>();
    }

    pub fn slice(&self, start: usize, end: usize) -> &[u8] {
        &self.raw_buffer[(self.offset + start)..(self.offset + end)]
    }

    pub fn duplicate(self, offset: usize) -> Self {
        Self {
            raw_buffer: self.raw_buffer,
            offset,
        }
    }

    pub fn zlib_decompress(&self, comp_length: usize) -> Self {
        let comp_buffer = self.slice(0, comp_length);
        let mut orig_buffer: Vec<u8> = Vec::new();
        match ZlibDecoder::new(comp_buffer).read_to_end(&mut orig_buffer) {
            Ok(_) => Self::new(orig_buffer),
            Err(_) => Self::new(comp_buffer.to_vec()),
        }
    }

    pub fn gz_decompress(&self, comp_length: usize) -> Self {
        let comp_buffer = self.slice(0, comp_length);
        let mut orig_buffer: Vec<u8> = Vec::new();
        match GzDecoder::new(comp_buffer).read_to_end(&mut orig_buffer) {
            Ok(_) => Self::new(orig_buffer),
            Err(_) => Self::new(comp_buffer.to_vec()),
        }
    }

    // pub fn calc_checksum(&self, offset: u32, length: u32) -> u32 {
    //     let _offset = offset as usize;
    //     let padded_length = ((length + 3) & !3) as usize;
    //     (0..padded_length).step_by(4).fold(0, |acc, i| {
    //         acc.wrapping_add(BigEndian::read_u32(
    //             &self.buffer[_offset + i.._offset + i + 4],
    //         ))
    //     })
    // }
}

impl fmt::Debug for Buffer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Buffer {{len: {}, elems: [{}, ..., {}]}}",
            self.raw_buffer.len(),
            self.raw_buffer.first().unwrap(),
            self.raw_buffer.last().unwrap(),
        )
    }
}

pub trait ReadBuffer {
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
/// - `FWord` = `int16`
/// - `UFWord` = `uint16`
/// - `F2Dot14`
/// - `LongDateTime`
/// - `Tag`
/// - `Offset16`
/// - `Offset32`

impl ReadBuffer for u8 {
    fn read(buffer: &mut Buffer) -> Self {
        let offset = buffer.offset;
        buffer.offset += mem::size_of::<u8>();
        buffer.raw_buffer[offset]
    }
}

impl ReadBuffer for i8 {
    fn read(buffer: &mut Buffer) -> Self {
        let offset = buffer.offset;
        buffer.offset += mem::size_of::<i8>();
        buffer.raw_buffer[offset] as i8
    }
}

// Implement `ReadBuffer` for `u16`, `u32`, etc.
macro_rules! _generate_read {
    ($t:ty, $f:expr) => {
        impl ReadBuffer for $t {
            fn read(buffer: &mut Buffer) -> Self {
                let offset = buffer.offset;
                buffer.offset += mem::size_of::<$t>();
                $f(&buffer.raw_buffer[offset..buffer.offset])
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
#[derive(Clone, Copy, ReadBuffer)]
pub struct u24 {
    _1: u16,
    _0: u8,
}

impl fmt::Debug for u24 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", usize::from(*self))
    }
}

impl From<u24> for usize {
    fn from(num: u24) -> Self {
        ((num._1 as usize) << 8) + (num._0 as usize)
    }
}

/// 32-bit signed fixed-point number (16.16).
#[derive(Default, ReadBuffer)]
pub struct Fixed {
    _num: i32,
}

impl fmt::Debug for Fixed {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:.3}", f64::from(self._num) / 65536.0)
    }
}

impl PartialEq<i32> for Fixed {
    fn eq(&self, other: &i32) -> bool {
        self._num == *other
    }
}

/// 16-bit signed fixed number with the low 14 bits of fraction (2.14).
#[derive(Clone, Copy, Default, ReadBuffer)]
pub struct F2Dot14 {
    _num: i16,
}

impl fmt::Debug for F2Dot14 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:.3}", self._num as f64 / 16384.0)
    }
}

impl PartialEq<i16> for F2Dot14 {
    fn eq(&self, other: &i16) -> bool {
        self._num == *other
    }
}

/// Date represented in number of seconds since 12:00 midnight, January 1, 1904.
/// The value is represented as a signed 64-bit integer.
#[derive(ReadBuffer)]
pub struct LongDateTime {
    _num: i64,
}

impl LongDateTime {
    /// Seconds from 1904-01-01 to 1970-01-01 (at midnight).
    const DATE_TIME_OFFSET: i64 = 2_082_844_800;
}

impl fmt::Debug for LongDateTime {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let timestamp = self._num - Self::DATE_TIME_OFFSET;
        write!(f, "{}", NaiveDateTime::from_timestamp(timestamp, 0))
    }
}

/// Array of four `uint8`s (length = 32 bits) used to identify a table,
/// design-variation axis, script, language system, feature, or baseline.
///
/// **Note:** In Rust, `char` is a *Unicode scalar value* with a size of 4 bytes
/// rather than 1, so it can't be used here.
#[derive(Default, Eq, PartialEq, Hash)]
pub struct Tag {
    _internal: [u8; 4],
}

impl Tag {
    pub fn from(tag_str: &str) -> Self {
        let mut bytes = tag_str.bytes();
        Self {
            _internal: [
                bytes.next().unwrap(),
                bytes.next().unwrap(),
                bytes.next().unwrap(),
                bytes.next().unwrap(),
            ],
        }
    }

    pub fn to_str(&self) -> &str {
        str::from_utf8(&self._internal).unwrap()
    }
}

impl ReadBuffer for Tag {
    fn read(buffer: &mut Buffer) -> Self {
        Self {
            _internal: [buffer.get(), buffer.get(), buffer.get(), buffer.get()],
        }
    }
}

impl fmt::Debug for Tag {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "\"{}\"", self.to_str())
    }
}

impl fmt::Display for Tag {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "\"{}\"", self.to_str())
    }
}
