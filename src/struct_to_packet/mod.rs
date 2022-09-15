mod tests;
use postcard::{to_allocvec};
use serde::{Serialize};
use std::convert::TryInto;

fn to_command_bytes(to_convert: &impl Serialize) -> Vec<u8> {
    return to_allocvec(to_convert).unwrap();
}

pub fn struct_to_packet_u16(to_convert: &impl Serialize) -> Vec<u8> {
    let mut command_packet_bytes = to_command_bytes(to_convert);
    let command_length: u16 = command_packet_bytes.len().try_into().unwrap();
    let mut command_length_bytes = command_length.to_le_bytes().to_vec();
    command_length_bytes.append(&mut command_packet_bytes);
    return command_length_bytes;
}

pub fn struct_to_packet_u32(to_convert: &impl Serialize) -> Vec<u8> {
    let mut command_packet_bytes = to_command_bytes(to_convert);
    let command_length: u32 = command_packet_bytes.len().try_into().unwrap();
    let mut command_length_bytes = command_length.to_le_bytes().to_vec();
    command_length_bytes.append(&mut command_packet_bytes);
    return command_length_bytes;
}