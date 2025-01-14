use core::fmt;

use crate::crc;

pub struct RdtPacket {
    payload: Vec<u8>,
    crc_byte: u8,
}

impl RdtPacket {
    fn new(payload: &String) -> Self {
        let payload_as_bytes = payload.as_bytes().to_vec();
        let crc_byte = crc::calculate_crc8_ccitt(&payload_as_bytes);
        return Self {
            payload: payload_as_bytes,
            crc_byte,
        };
    }

    pub fn validate_crc_byte(&self) -> bool {
        crc::calculate_crc8_ccitt(&self.payload) == self.crc_byte
    }
}

impl Into<Vec<u8>> for RdtPacket {
    fn into(self) -> Vec<u8> {
        let mut packet_bytes: Vec<u8> = vec![];
        packet_bytes.extend(self.payload);
        packet_bytes.push(self.crc_byte);
        packet_bytes
    }
}

impl TryFrom<&mut Vec<u8>> for RdtPacket {
    type Error = RdtPacketError;

    fn try_from(bytes: &mut Vec<u8>) -> Result<Self, Self::Error> {
        if let Some(crc_byte) = bytes.pop() {
            Ok(Self {
                payload: bytes.to_vec(),
                crc_byte,
            })
        } else {
            Err(RdtPacketError::MissingCrcByte)
        }
    }
}

#[derive(Debug)]
pub enum RdtPacketError {
    MissingCrcByte,
}

impl std::error::Error for RdtPacketError {}

impl fmt::Display for RdtPacketError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Error: {}",
            match *self {
                RdtPacketError::MissingCrcByte => "Failed to pop CRC byte.",
            }
        )
    }
}
