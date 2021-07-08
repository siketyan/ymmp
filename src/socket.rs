use std::convert::TryFrom;
use std::net::{SocketAddr, ToSocketAddrs, UdpSocket};

use crate::{Octet, Octets, Packet};

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("I/O error: {0}")]
    IoError(#[from] std::io::Error),
}

type Result<T> = std::result::Result<T, Error>;

pub struct YmmpSocket {
    udp: UdpSocket,
}

impl YmmpSocket {
    pub fn open<A: ToSocketAddrs>(addr: A) -> Result<Self> {
        Ok(Self {
            udp: UdpSocket::bind(addr).map_err(Error::IoError)?,
        })
    }

    pub fn send<A: ToSocketAddrs>(&self, packet: &Packet, to: A) -> Result<usize> {
        self.udp
            .send_to(&packet.to_octets_vec(), to)
            .map_err(Error::IoError)
    }

    pub fn receive(&self) -> Result<Packet> {
        let mut buffer: [Octet; 2048] = [0; 2048];
        let (read, _) = self.socket.recv_from(&mut buffer).map_err(Error::IoError)?;

        Packet::try_from(&buffer[..read] as &Octets).map_err(Error::PacketError)
    }
}
