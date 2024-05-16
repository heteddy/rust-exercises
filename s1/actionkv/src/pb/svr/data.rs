use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
use serde_derive::{Deserialize, Serialize};
// use std::convert::TryInto;

pub type ByteStr = [u8];
pub type ByteString = Vec<u8>;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeyValuePair {
    pub key: ByteString,
    pub value: ByteString,
}

impl KeyValuePair {
    pub fn new(key: ByteString, value: ByteString) -> Self {
        let mut d: Vec<u8> = vec![];
        d.write_u32::<LittleEndian>(300).unwrap(); // 这个用到的是byteorder trait
        print!("d: {:#x?}", &d);

        KeyValuePair { key, value }
    }
    pub fn dump(&self) -> ByteString {
        let mut tmp: Vec<u8> = ByteString::with_capacity(self.key.len() + self.value.len());
        for k in &self.key {
            tmp.push(*k);
        }
        for v in &self.value {
            tmp.push(*v);
        }

        return tmp;
    }
}

// let d: u32 = 300;
// let v = d.to_be_bytes().to_vec();
// print!("v: {:#x?}", &v);

// // u32 转大端 使用 byteorder
// let mut d: Vec<u8> = vec![];
// d.write_u32::<BigEndian>(300).unwrap();
// print!("d: {:#x?}", &d);
