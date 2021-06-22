use std::convert::TryFrom;
use std::net::{ToSocketAddrs, UdpSocket};

use crate::{Octet, Octets, Packet};

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("I/O error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("Packet error: {0}")]
    PacketError(#[from] crate::packet::Error),
}

type Result<T> = std::result::Result<T, Error>;

pub struct Receiver {
    socket: UdpSocket,
}

impl Receiver {
    pub fn open<A: ToSocketAddrs>(addr: A) -> Result<Self> {
        Ok(Self {
            socket: UdpSocket::bind(addr).map_err(Error::IoError)?,
        })
    }

    pub fn receive(&self) -> Result<Packet> {
        let mut buffer: [Octet; 2048] = [0; 2048];
        let (read, _) = self.socket.recv_from(&mut buffer).map_err(Error::IoError)?;

        Packet::try_from(&buffer[..read] as &Octets).map_err(Error::PacketError)
    }
}

#[cfg(test)]
mod tests {
    use std::net::{Ipv4Addr, SocketAddr, SocketAddrV4};

    use crate::broadcaster::Broadcaster;
    use crate::packet::Packet;
    use crate::receiver::Receiver;

    #[test]
    fn broadcast_and_receive() {
        let addr = SocketAddrV4::new(Ipv4Addr::LOCALHOST, 11223);
        let target = SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::LOCALHOST, 22334));
        let broadcaster = Broadcaster::open(addr, target).unwrap();
        let receiver = Receiver::open(target).unwrap();
        let packet = Packet::new(vec![]);

        broadcaster.broadcast(&packet).unwrap();

        let received = receiver.receive().unwrap();

        assert_eq!(packet, received);
    }
}
