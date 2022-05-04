use std::net::SocketAddr;

use tokio::net::{ToSocketAddrs, UdpSocket};

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
    pub async fn open<A: ToSocketAddrs>(addr: A, target: SocketAddr) -> Result<Self> {
        Ok(Self {
            socket: UdpSocket::bind(addr).await.map_err(Error::IoError)?,
            target,
        })
    }

    pub async fn broadcast(&self, packet: &Packet) -> Result<usize> {
        self.socket
            .send_to(packet.to_octets_vec().as_slice(), self.target)
            .await
            .map_err(Error::IoError)
    }
}

#[cfg(test)]
mod tests {
    use std::net::{Ipv4Addr, SocketAddr, SocketAddrV4};

    use crate::{Broadcaster, Packet};

    #[tokio::test]
    async fn broadcast() {
        let port = SocketAddrV4::new(Ipv4Addr::LOCALHOST, 12345);
        let target = SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 255), 12345);
        let broadcaster = Broadcaster::open(port, SocketAddr::V4(target))
            .await
            .expect("failed");
        let packet = Packet::default();
        let sent = broadcaster.broadcast(&packet).await.expect("failed");

        assert_eq!(packet.to_octets_vec().len(), sent);
    }
}
