use std::convert::TryFrom;

use tokio::net::{ToSocketAddrs, UdpSocket};

use crate::{Octet, Octets, Packet};

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("I/O error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("Packet error: {0}")]
    PacketError(#[from] crate::packet::Error),
}

type Result<T> = std::result::Result<T, Error>;

pub struct YmmpSocket {
    udp: UdpSocket,
}

impl YmmpSocket {
    pub async fn open<A: ToSocketAddrs>(addr: A) -> Result<Self> {
        Ok(Self {
            udp: UdpSocket::bind(addr).await.map_err(Error::IoError)?,
        })
    }

    pub async fn send<A: ToSocketAddrs>(&self, packet: &Packet, to: A) -> Result<usize> {
        self.udp
            .send_to(&packet.to_octets_vec(), to)
            .await
            .map_err(Error::IoError)
    }

    pub async fn receive(&self) -> Result<Packet> {
        let mut buffer: [Octet; 2048] = [0; 2048];
        let (read, _) = self
            .udp
            .recv_from(&mut buffer)
            .await
            .map_err(Error::IoError)?;

        Packet::try_from(&buffer[..read] as &Octets).map_err(Error::PacketError)
    }
}
