//! The following data types are used in the OpenType font file. All OpenType
//! fonts use Motorola-style byte ordering (Big Endian):
//!
//! - [`u8`]
//! - [`i8`]
//! - [`u16`]
//! - [`i16`]
//! - [`u24`]
//! - [`u32`]
//! - [`i32`]
//! - [`Fixed`]
//! - `FWord` = [`i16`]
//! - `UFWord` = [`u16`]
//! - [`F2Dot14`]
//! - [`LongDateTime`]
//! - [`Tag`]
//! - `Offset16` = [`u16`]
//! - `Offset32` = [`u32`]
//!
//! See: <https://docs.microsoft.com/en-us/typography/opentype/spec/otff#data-types>.

use crate::util::{Buffer, ReadBuffer};
use chrono::NaiveDateTime;
use read_buffer_derive::ReadBuffer;
use std::convert::TryInto;
use std::fmt;
use std::str;

/// 24-bit unsigned integer.
#[allow(non_camel_case_types)]
#[derive(Clone, Copy, Default, ReadBuffer)]
pub struct u24(u16, u8);

impl fmt::Debug for u24 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", usize::from(*self))
    }
}

impl From<u24> for usize {
    fn from(num: u24) -> Self {
        ((num.0 as usize) << 8) + (num.1 as usize)
    }
}

/// 32-bit signed fixed-point number (16.16).
#[derive(Clone, Copy, Default, ReadBuffer)]
pub struct Fixed(i32);

impl fmt::Debug for Fixed {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:.3}", f64::from(self.0) / 65536.0)
    }
}

impl PartialEq<i32> for Fixed {
    fn eq(&self, other: &i32) -> bool {
        self.0 == *other
    }
}

/// 16-bit signed fixed number with the low 14 bits of fraction (2.14).
#[derive(Clone, Copy, Default, ReadBuffer)]
pub struct F2Dot14(i16);

impl fmt::Debug for F2Dot14 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:.3}", self.0 as f64 / 16384.0)
    }
}

impl PartialEq<i16> for F2Dot14 {
    fn eq(&self, other: &i16) -> bool {
        self.0 == *other
    }
}

/// Date represented in number of seconds since 12:00 midnight, January 1, 1904.
/// The value is represented as a signed 64-bit integer.
#[derive(ReadBuffer)]
pub struct LongDateTime {
    num: i64,
}

impl LongDateTime {
    /// Seconds from 1904-01-01 to 1970-01-01 (at midnight).
    const DATE_TIME_OFFSET: i64 = 2_082_844_800;
}

impl fmt::Debug for LongDateTime {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let timestamp = self.num - Self::DATE_TIME_OFFSET;
        write!(f, "{}", NaiveDateTime::from_timestamp(timestamp, 0))
    }
}

/// Array of four `u8`s (length = 32 bits) used to identify a table,
/// design-variation axis, script, language system, feature, or baseline.
///
/// **Note:** In Rust, `char` is a *Unicode scalar value* with a size of 4 bytes
/// rather than 1, so it can't be used here.
#[derive(Clone, Copy, Default, Eq, PartialEq)]
pub struct Tag([u8; 4]);

impl Tag {
    /// Construct a tag from a `u8` array `bytes` with exactly 4 elements.
    ///
    /// # Examples
    ///
    /// ```
    /// # use rustotf::Tag;
    /// let tag = Tag::new(b"head");
    /// ```
    pub const fn new(bytes: &[u8; 4]) -> Self {
        Self(*bytes)
    }

    /// Return the underlying `u8` array of the tag.
    ///
    /// # Examples
    ///
    /// ```
    /// # use rustotf::Tag;
    /// let tag = Tag::from("OS/2");
    /// assert_eq!(tag.bytes(), b"OS/2");
    /// ```
    pub const fn bytes(&self) -> &[u8; 4] {
        &self.0
    }

    /// Convert the tag to a string.
    ///
    /// # Examples
    ///
    /// ```
    /// # use rustotf::Tag;
    /// let tag = Tag::new(b"glyf");
    /// assert_eq!(tag.to_str(), "glyf");
    /// ```
    pub fn to_str(&self) -> &str {
        str::from_utf8(&self.0).unwrap()
    }
}

impl From<u32> for Tag {
    /// Construct a tag from a 32-bit unsigned integer `n`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use rustotf::Tag;
    /// let tag = Tag::from(0x63767420);
    /// assert_eq!(tag, Tag::new(b"cvt "));
    /// ```
    fn from(n: u32) -> Self {
        let bytes = &[
            (n >> 24 & 0xFF) as u8,
            (n >> 16 & 0xFF) as u8,
            (n >> 8 & 0xFF) as u8,
            (n & 0xFF) as u8,
        ];
        Tag::new(bytes)
    }
}

impl From<&str> for Tag {
    /// Construct a tag from a string `s`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use rustotf::Tag;
    /// let tag = Tag::from("gsub");
    /// assert_eq!(tag, Tag::new(b"gsub"));
    /// ```
    ///
    /// # Panics
    ///
    /// Panic if the length of `s` is not 4.
    ///
    /// ```should_panic
    /// # use rustotf::Tag;
    /// let tag_cff = Tag::from("CFF"); // should use "CFF "
    /// ```
    fn from(s: &str) -> Self {
        let bytes = s.as_bytes().try_into().unwrap();
        Tag::new(bytes)
    }
}

impl PartialEq<&[u8; 4]> for Tag {
    fn eq(&self, other: &&[u8; 4]) -> bool {
        self.0 == **other
    }
}

impl fmt::Debug for Tag {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "\"{}\"", self.to_str())
    }
}

impl fmt::Display for Tag {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_str())
    }
}

impl ReadBuffer for Tag {
    fn read(buffer: &mut Buffer) -> Self {
        Self([buffer.get(), buffer.get(), buffer.get(), buffer.get()])
    }
}

/// `255UInt16` in WOFF2 specification. Variable-length encoding of a 16-bit unsigned integer
/// for optimized intermediate font data storage.
#[allow(non_camel_case_types)]
pub struct u16_var(u16);

impl fmt::Debug for u16_var {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// `UIntBase128` in WOFF2 specification. Variable-length encoding of a 32-bit unsigned integer
/// for optimized intermediate font data storage.
#[allow(non_camel_case_types)]
#[derive(Clone, Copy, Default)]
pub struct u32_var(u32);

impl PartialEq<u32> for u32_var {
    fn eq(&self, other: &u32) -> bool {
        self.0 == *other
    }
}

impl fmt::Debug for u32_var {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl ReadBuffer for u32_var {
    fn read(buffer: &mut Buffer) -> Self {
        let mut res = 0;
        for i in 0..5 {
            let byte: u8 = buffer.get();
            // No leading 0's
            if i == 0 && byte == 0x80 {
                panic!()
            }
            // If any of top 7 bits are set then << 7 would overflow
            if res & 0xFE00_0000 != 0 {
                panic!()
            }
            res = (res << 7) | ((byte & 0x7F) as u32);
            // Spin until most significant bit of data byte is false
            if byte & 0x80 == 0 {
                return Self(res);
            }
        }
        panic!()
    }
}

impl From<u32_var> for u32 {
    fn from(n: u32_var) -> Self {
        n.0
    }
}

#[test]
fn test_u32_var_success() {
    let mut buffer = Buffer::new(vec![0x3F]);
    assert_eq!(buffer.get::<u32_var>(), 63);
}

#[test]
#[should_panic]
fn test_u32_var_panic() {
    let mut buffer = Buffer::new(vec![0x80, 0x3F]);
    assert_eq!(buffer.get::<u32_var>(), 63);
}
