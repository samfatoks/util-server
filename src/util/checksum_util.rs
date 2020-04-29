use crc16::*;
use crc32fast::Hasher;

pub fn crc16(data: &str) -> String {
    let value = State::<CCITT_FALSE>::calculate(data.as_bytes());
    format!("{:#X}", value)
}

pub fn crc32(data: &str) -> String {
    let mut hasher = Hasher::new();
    hasher.update(data.as_bytes());
    format!("{:#X}", hasher.finalize())
}
