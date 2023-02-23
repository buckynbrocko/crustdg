use std::io::{Bytes, Stdin};

use crate::bitmask::BitMask;

// pub const SUBCODE_MASK: BitMask = 0x3F;
pub const SUBCODE_CDG_COMMAND_MARKER: BitMask = 0x09;

pub type SubCodePacket = [u8; 24];
pub type SubCodeData = [u8; 16];

#[derive(Debug, Clone, Copy)]
pub struct SubCodeStruct {
    pub command: u8,
    pub instruction: u8,
    pub parity_q: [u8; 2],
    pub data: [u8; 16],
    pub parity_p: [u8; 4],
}

impl From<SubCodePacket> for SubCodeStruct {
    fn from(packet: SubCodePacket) -> SubCodeStruct {
        SubCodeStruct {
            command: packet[0],
            instruction: packet[1],
            parity_q: packet[2..=3].try_into().unwrap(),
            data: packet[4..=20].try_into().unwrap(),
            parity_p: packet[21..].try_into().unwrap(),
        }
    }
}

pub trait Generator<T> {
    fn next(&mut self) -> T;
}

pub struct PacketGenerator {
    bytes: Bytes<Stdin>,
    buffer: [u8; 24],
}

impl From<Bytes<Stdin>> for PacketGenerator {
    fn from(bytes: Bytes<Stdin>) -> PacketGenerator {
        PacketGenerator {
            bytes,
            buffer: [0; 24],
        }
    }
}

impl Iterator for PacketGenerator {
    type Item = SubCodePacket;
    fn next(&mut self) -> Option<Self::Item> {
        for index in 0..24 {
            match self.bytes.next() {
                Some(Ok(byte)) => self.buffer[index] = byte,
                _ => return None,
            };
        }
        return Some(self.buffer);
    }
}

pub fn data_pairs(data: SubCodeData) -> [[u8; 2]; 8] {
    let mut vector = Vec::<[u8; 2]>::new();
    for index in 0..data.len() {
        if (index % 2) != 0 {
            vector.push([data[index - 1], data[index]]);
        }
    }
    vector.try_into().unwrap()
}

#[cfg(test)]
mod tests {
    #[allow(unused)]
    use super::data_pairs;
    use rstest::rstest;

    #[rstest]
    fn pairing() {
        let data = [0; 16];
        let paired = data_pairs(data);
        for (index, pair) in paired.iter().enumerate() {
            assert_eq!(pair, &[data[2 * index], data[2 * index + 1]]);
        }
    }
}
