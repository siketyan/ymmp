use std::net::{SocketAddr, ToSocketAddrs, UdpSocket};

use crate::Packet;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("I/O error: {0}")]
    IoError(#[from] std::io::Error),
}

type Result<T> = std::result::Result<T, Error>;

pub struct Broadcaster {
    socket: UdpSocket,
    target: SocketAddr,
}

impl Broadcaster {
    pub fn open<A: ToSocketAddrs>(addr: A, target: SocketAddr) -> Result<Self> {
        Ok(Self {
            socket: UdpSocket::bind(addr).map_err(Error::IoError)?,
            target,
        })
    }

    pub fn broadcast(&self, packet: &Packet) -> Result<usize> {
        self.socket
            .send_to(packet.to_octets_vec().as_slice(), self.target)
            .map_err(Error::IoError)
    }
}

#[cfg(test)]
mod tests {
    use std::net::{Ipv4Addr, SocketAddr, SocketAddrV4};

    use crate::{Broadcaster, Packet};

    #[test]
    fn broadcast() {
        let port = SocketAddrV4::new(Ipv4Addr::LOCALHOST, 12345);
        let target = SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 255), 12345);
        let broadcaster = Broadcaster::open(port, SocketAddr::V4(target)).expect("failed");
        let packet = Packet::default();
        let sent = broadcaster.broadcast(&packet).expect("failed");

        assert_eq!(packet.to_octets_vec().len(), sent);
    }
}
