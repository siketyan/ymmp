use crate::header::Header;
use crate::Octet;

pub struct Packet {
    header: Header,
    message: Vec<Octet>,
}

impl Packet {
    fn new(payload: Vec<Octet>) -> Self {
        Self {
            header: Header::default(),
            message: payload,
        }
    }

    pub fn to_octets_vec(&self) -> Vec<Octet> {
        let mut vec: Vec<u8> = self.header.to_octets_vec();

        vec.extend_from_slice(&(self.message.len() as u64).to_le_bytes());
        vec.extend_from_slice(self.message.as_slice());

        vec
    }
}

impl Default for Packet {
    fn default() -> Self {
        Self::new(Vec::new())
    }
}

#[cfg(test)]
mod tests {
    use crate::packet::Packet;

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
}
