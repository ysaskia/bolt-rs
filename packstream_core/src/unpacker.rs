use crate::consts::*;
use crate::error::BoltError;
use std::collections::HashMap;

pub struct PackInput {
    pub buf: Vec<u8>,
    index: usize,
}

impl PackInput {
    pub fn new(buf: Vec<u8>) -> Self {
        PackInput {
            buf,
            index: 0,
        }
    }

    pub fn len(self) -> usize {
        self.buf.len()
    }

    pub fn clear(&mut self) {
        println!("unpacker clear");
        self.buf.clear();
        self.index = 0;
    }

    pub fn extend(&mut self, size: usize) -> &mut [u8] {
        let start = self.buf.len();
        let end = start + size;
        self.buf.resize(end, 0);
        &mut self.buf[start..end]
    }

    pub fn peek(&mut self) -> Result<u8, BoltError> {
        self.buf
            .get(self.index)
            .map(|n| *n)
            .ok_or(BoltError::PackInputPeekInvalidRange)
    }

    pub fn peek_at(&mut self, i: usize) -> Result<u8, BoltError> {
        self.buf
            .get(self.index + i)
            .map(|n| *n)
            .ok_or(BoltError::PackInputPeekAtInvalidRange)
    }

    pub fn read_slice(&mut self, size: usize) -> Result<&[u8], BoltError> {
        let len = self.buf.len();
        let head = self.index;
        let last = head + size;

        if len > head && last <= len {
            self.index += size;
            Ok(&self.buf[head..last])
        } else {
            Err(BoltError::PackInputReadInvalidSliceRange)
        }
    }

    pub fn read_u8(&mut self) -> Result<u8, BoltError> {
        let out = self.buf
            .get(self.index)
            .map(|n| *n)
            .ok_or(BoltError::PackInputReadInvalidIndex);
        self.index += 1;
        out
    }

    fn read_i8(&mut self) -> Result<i8, BoltError> {
        self.read_u8().map(|x| x as i8)
    }

    fn read_i16(&mut self) -> Result<i16, BoltError> {
        SHIFT_16
            .iter()
            .fold(Ok(0), |n, x| n
                .and_then(|n| self
                    .read_u8()
                    .map(|b| (b as i16) << (*x as i16) | n)
                ),
            )
    }

    fn read_i32(&mut self) -> Result<i32, BoltError> {
        SHIFT_32
            .iter()
            .fold(Ok(0), |n, x| n
                .and_then(|n| self
                    .read_u8()
                    .map(|b| (b as i32) << (*x as i32) | n)
                ),
            )
    }

    fn read_i64(&mut self) -> Result<i64, BoltError> {
        SHIFT_64
            .iter()
            .fold(Ok(0), |n, x| n
                .and_then(|n| self
                    .read_u8()
                    .map(|b| (b as i64) << (*x as i64) | n)
                ),
            )
    }
}

pub struct Unpacker {
    pub input: PackInput,
}

impl Unpacker {
    pub fn new(buf: Vec<u8>) -> Self {
        Unpacker {
            input: PackInput::new(buf)
        }
    }

    pub fn len(self) -> usize {
        self.input.len()
    }

    pub fn clear(&mut self) {
        self.input.clear();
        println!("input {:?}", self.input.buf);

    }

    pub fn extend(&mut self, size: usize) -> &mut [u8] {
        self.input.extend(size)
    }

    fn unpack_string_header(&mut self) -> Result<i32, BoltError> {
        self.input
            .read_u8()
            .and_then(|byte| match byte {
                b if is(b, TINY_STRING) => Ok((b & LOW_NIBBLE) as i32),
                STRING_8 => self.input.read_i8().map(i32::from),
                STRING_16 => self.input.read_i16().map(i32::from),
                STRING_32 => self.input.read_i32().map(i32::from),
                _ => Err(BoltError::UnpackInvalidStringSize)
            })
    }

    fn unpack_map_header(&mut self) -> Result<i32, BoltError> {
        self.input
            .read_u8()
            .and_then(|byte| match byte {
                b if is(b, TINY_MAP) => Ok((b & LOW_NIBBLE) as i32),
                MAP_8 => self.input.read_i8().map(i32::from),
                MAP_16 => self.input.read_i16().map(i32::from),
                MAP_32 => self.input.read_i32(),
                MAP_STREAM => Ok(UNKNOWN_SIZE),
                _ => Err(BoltError::UnpackInvalidMapSize)
            })
    }

    fn unpack_list_header(&mut self) -> Result<i32, BoltError> {
        self.input
            .read_u8()
            .and_then(|byte| {
                if is(byte, TINY_LIST) {
                    Ok((byte & LOW_NIBBLE) as i32)
                }
                else {
                    match byte {
                        LIST_8      => self.input.read_i8().map(i32::from),
                        LIST_16     => self.input.read_i16().map(i32::from),
                        LIST_32     => self.input.read_i32(),
                        LIST_STREAM => Ok(UNKNOWN_SIZE),
                        _           => Err(BoltError::UnpackInvalidListSize)
                    }
                }
            })
    }

    pub fn unpack_struct_header(&mut self) -> Result<i32, BoltError> {
        self.input
            .read_u8()
            .and_then(|byte| {
                if is(byte, TINY_STRUCT) {
                    Ok((byte & LOW_NIBBLE) as i32)
                }
                else {
                    match byte {
                        STRUCT_8  => self.input.read_i8().map(i32::from),
                        STRUCT_16 => self.input.read_i16().map(i32::from),
                        _         => Err(BoltError::UnpackInvalidStructSize)
                    }
                }
            })
    }

    pub fn peek_struct_signature(&mut self) -> Result<u8, BoltError> {
        self.input
            .peek()
            .and_then(|byte| {
                if is(byte, TINY_STRUCT) {
                    self.input.peek_at(1)
                }
                else {
                    match byte {
                        STRUCT_8  => self.input.peek_at(2),
                        STRUCT_16 => self.input.peek_at(3),
                        _         => Err(BoltError::UnpackInvalidStructSize)
                    }
                }
            })
    }

    fn unpack_map<T>(&mut self, s: usize) -> Result<HashMap<String, T>, BoltError>
        where Unpacker: UnpackValue<T> {
        let mut kvs = HashMap::with_capacity(s as usize);
        let mut out = Ok(());
        loop {
            match kvs.len() {
                i if i < s => out = out
                    .and_then(|_| (self as &mut dyn UnpackValue<String>).unpack())
                    .and_then(|k| (self).unpack().map(|v| (k, v)))
                    .map(|(k, v)| insert(&mut kvs, k, v)),
                _ => break out.map(|_| kvs)
            }
        }
    }

    fn unpack_map_stream<T>(&mut self) -> Result<HashMap<String, T>, BoltError>
        where Unpacker: UnpackValue<T> {
        let mut kvs = HashMap::new();
        let mut out = Ok(());
        loop {
            match self.input.read_u8() {
                Ok(b) if b != END_OF_STREAM => out = out
                    .and_then(|_| (self as &mut dyn UnpackValue<String>).unpack())
                    .and_then(|k| (self).unpack().map(|v| (k, v)))
                    .map(|(k, v)| insert(&mut kvs, k, v)),
                Ok(_) => break out.map(|_| kvs),
                Err(e) => break Err(e)
            }
        }
    }

    fn unpack_list<T>(&mut self, s: usize) -> Result<Vec<T>, BoltError>
        where Unpacker: UnpackValue<T> {
        let mut xs = Vec::with_capacity(s as usize);
        let mut out = Ok(());
        loop {
            match xs.len() {
                i if i < s => out = out
                    .and_then(|_| self.unpack())
                    .map(|x| xs.push(x)),
                _ => break out.map(|_| xs)
            }
        }
    }

    fn unpack_list_stream<T>(&mut self) -> Result<Vec<T>, BoltError>
        where Unpacker: UnpackValue<T> {
        let mut xs = Vec::new();
        let mut out = Ok(());
        loop {
            match self.input.read_u8() {
                Ok(b) if b != END_OF_STREAM => out = out
                    .and_then(|_| self.unpack())
                    .map(|x| xs.push(x)),
                Ok(_) => break out.map(|_| xs),
                Err(e) => break Err(e)
            }
        }
    }

    pub fn unpack_struct_signature(&mut self) -> Result<u8, BoltError> {
        self.input.read_u8()
    }
}

pub trait UnpackValue<T> {
    fn unpack(&mut self) -> Result<T, BoltError>;
}

impl UnpackValue<bool> for Unpacker {
    fn unpack(&mut self) -> Result<bool, BoltError> {
        self.input
            .read_u8()
            .and_then(|byte| match byte {
                TRUE => Ok(true),
                FALSE => Ok(false),
                _ => Err(BoltError::UnpackInvalidBooleanMarker)
            })
    }
}

impl UnpackValue<i8> for Unpacker {
    fn unpack(&mut self) -> Result<i8, BoltError> {
        self.unpack().map(|n: i64| n as i8)
    }
}

impl UnpackValue<i16> for Unpacker {
    fn unpack(&mut self) -> Result<i16, BoltError> {
        self.unpack().map(|n: i64| n as i16)
    }
}

impl UnpackValue<i32> for Unpacker {
    fn unpack(&mut self) -> Result<i32, BoltError> {
        self.unpack().map(|n: i64| n as i32)
    }
}

impl UnpackValue<i64> for Unpacker {
    fn unpack(&mut self) -> Result<i64, BoltError> {
        self.input
            .read_u8()
            .and_then(|byte| match byte {
                b @ TINY_INT_PF..=TINY_INT_PL => Ok(b as i64),
                b @ TINY_INT_NF..=TINY_INT_NL => Ok(b as i64 - 0x100),
                INT_8 => self.input.read_i8().map(i64::from),
                INT_16 => self.input.read_i16().map(i64::from),
                INT_32 => self.input.read_i32().map(i64::from),
                INT_64 => self.input.read_i64(),
                _ => Err(BoltError::UnpackInvalidIntMarker)
            })
    }
}

impl UnpackValue<f64> for Unpacker {
    fn unpack(&mut self) -> Result<f64, BoltError> {
        self.input
            .read_u8()
            .and_then(|_| self.input.read_i64().map(|n| f64::from_bits(n as u64)))
    }
}

impl UnpackValue<String> for Unpacker {
    fn unpack(&mut self) -> Result<String, BoltError> {
        self.unpack_string_header()
            .and_then(|byte| match byte {
                0 => Ok(String::new()),
                s => self
                    .input
                    .read_slice(s as usize)
                    .map(String::from_utf8_lossy)
                    .map(|cs| cs.into())
            })
    }
}

impl<T> UnpackValue<Vec<T>> for Unpacker
    where Unpacker: UnpackValue<T> {
    fn unpack(&mut self) -> Result<Vec<T>, BoltError> {
        self.unpack_list_header()
            .and_then(|byte| match byte {
                EMPTY_SIZE => Ok(Vec::new()),
                UNKNOWN_SIZE => self.unpack_list_stream(),
                s => self.unpack_list(s as usize)
            })
    }
}

impl<T> UnpackValue<HashMap<String, T>> for Unpacker
    where Unpacker: UnpackValue<T> {
    fn unpack(&mut self) -> Result<HashMap<String, T>, BoltError> {
        self.unpack_map_header()
            .and_then(|byte| match byte {
                EMPTY_SIZE   => Ok(HashMap::new()),
                UNKNOWN_SIZE => self.unpack_map_stream(),
                s            => self.unpack_map(s as usize)
            })
    }
}

fn is(byte: u8, marker: u8) -> bool {
    byte & HIGH_NIBBLE == marker
}

fn insert<K, V>(kvs: &mut HashMap<K, V>, key: K, value: V)
    where K: std::cmp::Eq, K: std::hash::Hash {
    kvs.insert(key, value);
}
