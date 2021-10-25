use byteorder::{BigEndian, ByteOrder};
use flate2::read::{GzDecoder, ZlibDecoder};
use std::fmt;
use std::io::{Read, Result};
use std::mem;

pub struct Buffer {
    bytes: Vec<u8>,
    offset: usize,
}

impl Buffer {
    /// Create a new `Buffer`.
    pub fn new(bytes: Vec<u8>) -> Self {
        Self { bytes, offset: 0 }
    }

    /// Return the length of the buffer.
    pub fn len(&self) -> usize {
        self.bytes.len()
    }

    /// Get a value as type `T` from the buffer.
    pub fn get<T: ReadBuffer>(&mut self) -> T {
        ReadBuffer::read(self)
    }

    /// Get a vector of type `T` values from the buffer.
    pub fn get_vec<T: ReadBuffer, N: AsUsize>(&mut self, n: N) -> Vec<T> {
        (0..n.as_usize()).map(|_| ReadBuffer::read(self)).collect()
    }

    /// Get an option of type `T` values from the buffer.
    /// If `offset` is 0 (i.e. NULL), then it will return a `None`.
    pub fn get_or_none<T: ReadBuffer, N: AsUsize>(&mut self, start: usize, offset: N) -> Option<T> {
        match offset.as_usize() {
            0 => None,
            offset => {
                // offset != 0
                self.offset = start + offset;
                Some(self.get::<T>())
            }
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

    /// Return the offset of the buffer.
    pub fn offset(&self) -> usize {
        self.offset
    }

    pub fn set_offset<N: AsUsize>(&mut self, offset: N) {
        self.offset = offset.as_usize()
    }

    pub fn advance_offset<N: AsUsize>(&mut self, offset: N) {
        self.offset += offset.as_usize()
    }

    pub fn set_offset_from<N: AsUsize>(&mut self, start: usize, offset: N) {
        self.offset = start + offset.as_usize()
    }

    pub fn slice(&self, start: usize, end: usize) -> &[u8] {
        &self.bytes[(self.offset + start)..(self.offset + end)]
    }

    pub fn zlib_decompress(&self, comp_len: usize) -> Result<Self> {
        let comp_buffer = self.slice(0, comp_len);
        let mut orig_buffer = Vec::new();
        ZlibDecoder::new(comp_buffer).read_to_end(&mut orig_buffer)?;
        Ok(Self::new(orig_buffer))
    }

    pub fn gz_decompress(&self, comp_len: usize) -> Result<Self> {
        let comp_buffer = self.slice(0, comp_len);
        let mut orig_buffer = Vec::new();
        GzDecoder::new(comp_buffer).read_to_end(&mut orig_buffer)?;
        Ok(Self::new(orig_buffer))
    }

    // pub fn calc_checksum(&self, offset: u32, length: u32) -> u32 {
    //     let offset = offset as usize;
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
            self.bytes.len(),
            self.bytes.first().unwrap(),
            self.bytes.last().unwrap(),
        )
    }
}

pub trait AsUsize {
    fn as_usize(&self) -> usize;
}

macro_rules! generate_as_usize {
    ($t:ty) => {
        impl AsUsize for $t {
            fn as_usize(&self) -> usize {
                *self as usize
            }
        }
    };
}

generate_as_usize!(u8);
generate_as_usize!(u16);
generate_as_usize!(u32);
generate_as_usize!(u64);
generate_as_usize!(usize);
generate_as_usize!(i8);
generate_as_usize!(i16);
generate_as_usize!(i32);
generate_as_usize!(i64);

pub trait ReadBuffer {
    fn read(buffer: &mut Buffer) -> Self;
}

impl ReadBuffer for u8 {
    fn read(buffer: &mut Buffer) -> Self {
        let offset = buffer.offset();
        buffer.offset += mem::size_of::<u8>();
        buffer.bytes[offset]
    }
}

impl ReadBuffer for i8 {
    fn read(buffer: &mut Buffer) -> Self {
        let offset = buffer.offset();
        buffer.offset += mem::size_of::<i8>();
        buffer.bytes[offset] as i8
    }
}

/// Implement `ReadBuffer` for `u16`, `u32`, etc.
macro_rules! generate_read {
    ($t:ty, $f:expr) => {
        impl ReadBuffer for $t {
            fn read(buffer: &mut Buffer) -> Self {
                let offset = buffer.offset();
                buffer.offset += mem::size_of::<$t>();
                $f(&buffer.bytes[offset..buffer.offset])
            }
        }
    };
}

generate_read!(u16, BigEndian::read_u16);
generate_read!(u32, BigEndian::read_u32);
generate_read!(u64, BigEndian::read_u64);
generate_read!(i16, BigEndian::read_i16);
generate_read!(i32, BigEndian::read_i32);
generate_read!(i64, BigEndian::read_i64);
