use crustdg::prelude::Command;
use crustdg::prelude::SubCodePacket;
use std::io::*;
// use std::str::Bytes;

fn main() {
    let mut buffer: Vec<u8> = Vec::new();
    let bytes = stdin().bytes();
    for byte in bytes {
        buffer.push(byte.unwrap());

        if buffer.len() == 24 {
            let packet: SubCodePacket = buffer.try_into().unwrap();
            buffer = Vec::new();
            match Command::try_from(packet) {
                Ok(command) => {
                    println!("{:?}", command);
                },
                Err(_) => {},
            }
        }
    }
}

