use crate::Octet;

pub(crate) const MAGIC_LENGTH: usize = 4;
pub(crate) const VERSION_LENGTH: usize = 4;
pub(crate) const HEADER_LENGTH: usize = MAGIC_LENGTH + VERSION_LENGTH;

pub(crate) type Magic = [Octet; MAGIC_LENGTH];
pub(crate) type Version = [Octet; VERSION_LENGTH];
pub(crate) type HeaderOctets = [Octet; HEADER_LENGTH];

#[derive(Debug, PartialEq)]
pub struct Header {
    magic: Magic,
    version: Version,
}

const MAGIC: Magic = [b'Y', b'M', b'M', b'P'];
const VERSION: Version = [b'v', b'1', b'.', b'0'];

impl Header {
    pub fn from_raw_parts(magic: &Magic, version: &Version) -> Self {
        Self {
            magic: magic.clone(),
            version: version.clone(),
        }
    }

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

impl From<&HeaderOctets> for Header {
    fn from(value: &HeaderOctets) -> Self {
        let mut magic = Magic::default();
        let mut version = Version::default();

        magic.copy_from_slice(&value[0..4]);
        version.copy_from_slice(&value[4..8]);

        Header::from_raw_parts(&magic, &version)
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
