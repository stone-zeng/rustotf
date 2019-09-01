// extern crate byteorder;

use std::fmt;

use byteorder::{BigEndian, ByteOrder};
use chrono::NaiveDateTime;

/// Seconds from 1904-01-01 to 1970-01-01 (at midnight)
const DATE_TIME_OFFSET: i64 = 2082844800;

/// 32-bit signed fixed-point number (16.16)
pub struct Fixed {
    num: i32,
}
impl fmt::Debug for Fixed {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:.3}", self.num as f64 / 65536.0)
    }
}
impl PartialEq<i32> for Fixed {
    fn eq(&self, other: &i32) -> bool {
        self.num == *other
    }
}

pub fn get_version_string(major: u16, minor: u16) -> String {
    major.to_string() + "." + &minor.to_string()
}

// type FWORD = i16;
// type UFWORD = u16;

pub struct Buffer {
    pub buffer: Vec<u8>,
    pub offset: u32,
}

impl Buffer {
    pub fn read_u8(&mut self) -> u8 {
        let _offset = self.offset as usize;
        let v = self.buffer[_offset];
        self.offset += 1;
        v
    }
    pub fn read_u16(&mut self) -> u16 {
        let _offset = self.offset as usize;
        let v = BigEndian::read_u16(&self.buffer[_offset.._offset + 2]);
        self.offset += 2;
        v
    }
    pub fn read_u32(&mut self) -> u32 {
        let _offset = self.offset as usize;
        let v = BigEndian::read_u32(&self.buffer[_offset.._offset + 4]);
        self.offset += 4;
        v
    }
    // pub fn read_u64(&mut self) -> u64 {
    //     let _offset = self.offset as usize;
    //     let v = BigEndian::read_u64(&self.buffer[_offset.._offset + 8]);
    //     self.offset += 8;
    //     v
    // }
    pub fn read_i16(&mut self) -> i16 {
        let _offset = self.offset as usize;
        let v = BigEndian::read_i16(&self.buffer[_offset.._offset + 2]);
        self.offset += 2;
        v
    }
    pub fn read_i32(&mut self) -> i32 {
        let _offset = self.offset as usize;
        let v = BigEndian::read_i32(&self.buffer[_offset.._offset + 4]);
        self.offset += 4;
        v
    }
    pub fn read_i64(&mut self) -> i64 {
        let _offset = self.offset as usize;
        let v = BigEndian::read_i64(&self.buffer[_offset.._offset + 8]);
        self.offset += 8;
        v
    }
    pub fn read_fixed(&mut self) -> Fixed {
        Fixed {
            num: self.read_i32(),
        }
    }
    pub fn read_datetime(&mut self) -> NaiveDateTime {
        NaiveDateTime::from_timestamp(self.read_i64() - DATE_TIME_OFFSET, 0)
    }
    pub fn read_tag(&mut self) -> String {
        let mut v = String::from("");
        for _ in 0..4 {
            v.push(self.read_u8() as char)
        }
        v
    }
    pub fn skip(&mut self, n: u32) {
        self.offset += n;
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
