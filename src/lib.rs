mod broadcaster;
mod header;
mod packet;

pub(crate) type Octet = u8;
pub(crate) type Octets = [Octet];

pub use crate::broadcaster::Broadcaster;
pub use crate::header::Header;
pub use crate::packet::Packet;
