use crate::Octet;

pub struct Header {
    magic: [Octet; 4],
    version: [Octet; 4],
}

const MAGIC: [Octet; 4] = [b'Y', b'M', b'M', b'P'];
const VERSION: [Octet; 4] = [b'v', b'1', b'.', b'0'];

impl Header {
    pub fn to_octets_vec(&self) -> Vec<Octet> {
        let mut vec = self.magic.clone().to_vec();

        vec.append(&mut self.version.clone().to_vec());

        vec
    }
}

impl Default for Header {
    fn default() -> Self {
        Self {
            magic: MAGIC,
            version: VERSION,
        }
    }
}

impl Into<[Octet; 8]> for Header {
    fn into(self) -> [Octet; 8] {
        [
            self.magic[0],
            self.magic[1],
            self.magic[2],
            self.magic[3],
            self.version[0],
            self.version[1],
            self.version[2],
            self.version[3],
        ]
    }
}

#[cfg(test)]
mod tests {
    use crate::header::Header;

    #[test]
    fn default() {
        let header = Header::default();
        let bytes = header.to_octets_vec();

        assert_eq!(8, bytes.len());
        assert_eq!(vec![b'Y', b'M', b'M', b'P', b'v', b'1', b'.', b'0'], bytes);
    }
}
