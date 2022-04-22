use std::convert::{TryFrom, TryInto};
use std::string::FromUtf8Error;

use crate::header::{Header, HeaderOctets, HEADER_LENGTH};
use crate::{Octet, Octets};

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Too short packet found.")]
    PacketTooShort,
}

#[derive(Debug, PartialEq)]
pub struct Packet {
    header: Header,
    message: Vec<Octet>,
}

impl Packet {
    pub fn new(message: Vec<Octet>) -> Self {
        Self {
            header: Header::default(),
            message,
        }
    }

    pub fn from_raw_parts(header: &HeaderOctets, message: &Octets) -> Self {
        Self {
            header: Header::from(header),
            message: message.to_vec(),
        }
    }

    pub fn to_octets_vec(&self) -> Vec<Octet> {
        let mut vec: Vec<u8> = self.header.to_octets_vec();

        vec.extend_from_slice(&(self.message.len() as u64).to_le_bytes());
        vec.extend_from_slice(self.message.as_slice());

        vec
    }

    pub fn message(&self) -> &[Octet] {
        &self.message
    }
}

impl Default for Packet {
    fn default() -> Self {
        Self::new(Vec::new())
    }
}

impl TryFrom<&Octets> for Packet {
    type Error = Error;

    fn try_from(value: &Octets) -> Result<Self, Self::Error> {
        if value.len() < HEADER_LENGTH + 8 {
            Err(Error::PacketTooShort)
        } else {
            let mut header = HeaderOctets::default();
            header.copy_from_slice(&value[0..HEADER_LENGTH]);

            let offset = HEADER_LENGTH + 8;
            let mut length: [Octet; 8] = [0; 8];
            length.copy_from_slice(&value[HEADER_LENGTH..offset]);

            let length = u64::from_le_bytes(length);
            let rest = &value[offset..];

            // TODO: Support Fragmentation
            assert_eq!(length as usize, rest.len());

            Ok(Packet::from_raw_parts(&header, rest))
        }
    }
}

impl TryInto<String> for Packet {
    type Error = FromUtf8Error;

    fn try_into(self) -> Result<String, Self::Error> {
        String::from_utf8(self.message)
    }
}

#[cfg(test)]
mod tests {
    use crate::packet::Packet;
    use crate::Octets;
    use std::convert::TryFrom;

    #[test]
    fn default() {
        let packet = Packet::default();
        let header_bytes = packet.header.to_octets_vec();
        let header_len = header_bytes.len();
        let bytes = packet.to_octets_vec();

        assert_eq!(header_len + 8, bytes.len());
        assert_eq!(&header_bytes, &bytes[..header_len])
    }

    #[test]
    fn with_message() {
        let message = vec![b'f', b'o', b'o'];
        let packet = Packet::new(message.clone());
        let header_bytes = packet.header.to_octets_vec();
        let header_len = header_bytes.len();
        let bytes = packet.to_octets_vec();

        assert_eq!(header_len + 8 + message.len(), bytes.len());
        assert_eq!(&header_bytes, &bytes[..header_len]);
        assert_eq!(&message, &bytes[header_len + 8..]);
    }

    #[test]
    fn try_from_header_only() {
        let octets = vec![
            b'Y', b'M', b'M', b'P', b'v', b'0', b'.', b'1', 0, 0, 0, 0, 0, 0, 0, 0,
        ];

        let octets: &Octets = octets.as_slice();
        let packet = Packet::try_from(octets).expect("failed");

        assert_eq!(0, packet.message.len())
    }
}
