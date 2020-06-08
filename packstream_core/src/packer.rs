use std::vec::Vec;
use std::collections::HashMap;
use std::ops::{Index,Range,RangeTo,RangeFrom,RangeFull};
use crate::consts::*;
use crate::error::BoltError;

pub struct PackOutput {
    pub buf: Vec<u8>
}

impl PackOutput {
    fn new() -> Self {
        PackOutput {
            buf: Vec::new()
        }
    }

    fn push_byte(&mut self, byte: u8) -> Result<(), BoltError> {
        Ok(self.buf.push(byte))
    }

    fn push_bytes(&mut self, bytes: &[u8]) -> Result<(), BoltError> {
        Ok(self.buf.extend_from_slice(&bytes))
    }
}

pub struct Packer {
    pub out: PackOutput
}

impl Packer {
    pub fn new() -> Self {
        Packer {
            out: PackOutput::new()
        }
    }

    pub fn len(&mut self) -> usize {
        self.out.buf.len()
    }

    pub fn pack_head3(
        &mut self,
        size: usize,
        h8: u8,
        h16: u8,
        h32: u8
    ) -> Result<(), BoltError> {
        match size {
            s if s <= std::i8::MAX as usize => self.out
                .push_byte(h8)
                .and_then(|_| self.out
                    .push_bytes(&(size as i8).to_be_bytes())
                ),
            s if s <= std::i16::MAX as usize => self.out
                .push_byte(h16)
                .and_then(|_| self.out
                    .push_bytes(&(size as i16).to_be_bytes())
                ),
            s if s <= std::i32::MAX as usize => self.out
                .push_byte(h32)
                .and_then(|_| self.out
                    .push_bytes(&(size as i32).to_be_bytes())
                ),
            _ => Err(BoltError::PackHeaderSizeOverflow)
        }
    }

    /**
     */
    pub fn pack_head4(
        &mut self,
        size: usize,
        h4: u8,
        h8: u8,
        h16: u8,
        h32: u8
    ) -> Result<(), BoltError> {
        if size <= TINY_MAX as usize {
            self.out.push_byte(h4 | (size as u8))
        }
        else {
            self.pack_head3(size, h8, h16, h32)
        }
    }

    /**
     */
    pub fn pack_struct_header(
        &mut self,
        size: usize,
        sign: u8
    ) -> Result<(), BoltError> {
        match size {
            s if s < 0x10 as usize => self.out
                .push_byte(TINY_STRUCT + s as u8)
                .and_then(|_| self.out.push_byte(sign)) ,
            s if s <= std::i8::MAX as usize => self.out
                .push_byte(STRUCT_8)
                .and_then(|_| self.pack(s as i8))
                .and_then(|_| self.out.push_byte(sign)),
            s if s <= std::i16::MAX as usize => self.out
                .push_byte(STRUCT_16)
                .and_then(|_| self.pack(s as i16))
                .and_then(|_| self.out.push_byte(sign)),
            _ => Err(BoltError::PackStructHeaderSizeOverflow)
        }
    }
}

pub trait PackValue<T> {
    fn pack(&mut self, val: T) -> Result<(), BoltError>;
}

impl<T> PackValue<Option<T>> for Packer
    where Packer: PackValue<T> {
    fn pack(&mut self, val: Option<T>) -> Result<(), BoltError> {
        if val.is_none() {
            self.out.push_byte(NULL)
        }
        else {
            self.pack(val.unwrap())
        }
    }
}

/// Pushes bool value to stream.
impl PackValue<bool> for Packer {
    fn pack(&mut self, b: bool) -> Result<(), BoltError> {
        if b { self.out.push_byte(TRUE)  }
        else { self.out.push_byte(FALSE) }
    }
}

/// Pushes 8-bit integer value to stream.
impl PackValue<i8> for Packer {
    fn pack(&mut self, n: i8) -> Result<(), BoltError> {
        if n >= TINY_MIN &&
            n <= std::i8::MAX {
            self.out.push_byte(n as u8)
        }
        else {
            self.out
                .push_byte(INT_8)
                .and_then(|_| self.out.push_byte(n as u8))
        }
    }
}

/// Pushes 16-bit integer value to the output stream.
impl PackValue<i16> for Packer {
    fn pack(&mut self, n: i16) -> Result<(), BoltError> {
        if n >= std::i8::MIN as i16 &&
            n <= std::i8::MAX as i16 {
            self.pack(n as i8)
        }
        else {
            self.out
                .push_byte(INT_16)
                .and_then(|_| self.out.push_bytes(&n.to_be_bytes()))
        }
    }
}

/// Pushes 32-bit integers
impl PackValue<i32> for Packer {
    fn pack(&mut self, n: i32) -> Result<(), BoltError> {
        if n >= std::i16::MIN as i32 &&
            n <= std::i16::MAX as i32 {
            self.pack(n as i16)
        }
        else {
            self.out
                .push_byte(INT_32)
                .and_then(|_| self.out.push_bytes(&n.to_be_bytes()))
        }
    }
}

/// Pushes 64-bit integers
impl PackValue<i64> for Packer {
    fn pack(&mut self, n: i64) -> Result<(), BoltError> {
        if n >= std::i32::MIN as i64 && n <= std::i32::MAX as i64 {
            self.pack(n as i32)
        }
        else {
            self.out
                .push_byte(INT_64)
                .and_then(|_| self.out.push_bytes(&n.to_be_bytes()))
        }
    }
}

/// Pushes 64-bit float value to the output stream.
impl PackValue<f64> for Packer {
    fn pack(&mut self, n: f64) -> Result<(), BoltError> {
        self.out
            .push_byte(FLOAT_64)
            .and_then(|_| self.out
                .push_bytes(&n.to_bits().to_be_bytes())
            )
    }
}

/// Pushes str value to the output stream.
impl<'a> PackValue<&'a str> for Packer {
    fn pack(&mut self, cs: &'a str) -> Result<(), BoltError> {
        self
            .pack_head4(
                cs.len(),
                TINY_STRING,
                STRING_8,
                STRING_16,
                STRING_32
            )
            .and_then(|_| self.out.push_bytes(cs.as_bytes()))
    }
}

/// Pushes String value to the output stream.
impl PackValue<String> for Packer {
    fn pack(&mut self, cs: String) -> Result<(), BoltError> {
        self
            .pack_head4(
                cs.len(),
                TINY_STRING,
                STRING_8,
                STRING_16,
                STRING_32
            )
            .and_then(|_| self.out.push_bytes(cs.as_bytes()))
    }
}

/// Pushes vec values as list of values to the output stream.
impl<T> PackValue<Vec<T>> for Packer
    where Packer: PackValue<T> {
    fn pack(&mut self, xs: Vec<T>) -> Result<(), BoltError> {
        self
            .pack_head4(
                xs.len(),
                TINY_LIST,
                LIST_8,
                LIST_16,
                LIST_32
            )
            .and_then(|_| xs
                .into_iter()
                .fold(Ok(()), |out, x| out.and_then(|_| self.pack(x)))
            )
    }
}

/// Pushes vec values an hash map of key value pair where key are strings
/// to the output stream.
impl<V> PackValue<HashMap<&str, V>> for Packer
    where Packer: PackValue<V> {
    fn pack(&mut self, dict: HashMap<&str, V>) -> Result<(), BoltError> {
        self
            .pack_head4(dict.len(), TINY_MAP, MAP_8, MAP_16, MAP_32)
            .and_then(|_| dict
                .into_iter()
                .fold(Ok(()), |out, (k,v)| out
                    .and_then(|_| (self as &mut dyn PackValue<&str>).pack(k))
                    .and_then(|_| (self).pack(v))
                )
            )
    }
}

/// Pushes vec values an hash map of key value pair where key are strings
/// to the output stream.
impl<V> PackValue<HashMap<String, V>> for Packer
    where Packer: PackValue<V> {
    fn pack(&mut self, dict: HashMap<String, V>) -> Result<(), BoltError> {
        self
            .pack_head4(dict.len(), TINY_MAP, MAP_8, MAP_16, MAP_32)
            .and_then(|_| dict
                .into_iter()
                .fold(Ok(()), |r, (k,v)| r
                    .and_then(|_| (self as &mut dyn PackValue<String>).pack(k))
                    .and_then(|_| (self).pack(v))
                )
            )
    }
}

impl Index<usize> for Packer {
    type Output = u8;

    fn index(&self, index: usize) -> &Self::Output {
        &self.out.buf[index]
    }
}

impl Index<Range<usize>> for Packer {
    type Output = [u8];

    fn index(&self, index: Range<usize>) -> &Self::Output {
        &self.out.buf[index.start..index.end]
    }
}

impl Index<RangeTo<usize>> for Packer {
    type Output = [u8];

    fn index(&self, index: RangeTo<usize>) -> &Self::Output {
        &self.out.buf[..index.end]
    }
}

impl Index<RangeFrom<usize>> for Packer {
    type Output = [u8];

    fn index(&self, index: RangeFrom<usize>) -> &Self::Output {
        &self.out.buf[index.start..]
    }
}

impl Index<RangeFull> for Packer {
    type Output = [u8];

    fn index(&self, _: RangeFull) -> &[u8] {
        &self.out.buf[..]
    }
}
