use core::f64;
use std::{io, net::{SocketAddr, UdpSocket}};

use rand::Rng;

enum VirtualSocketError {
    Drop,
    Delay,
    BitError
}

/// A virtual socket that simulates errors happening in udp packet transfer.
pub struct VirtualSocket {
    socket: UdpSocket,
}

impl VirtualSocket {
    /// Create a virtual socket from the gived address.
    pub fn bind(socket_addr: SocketAddr) -> Result<Self, std::io::Error> {
        let socket: UdpSocket = match UdpSocket::bind(socket_addr) {
            Ok(socket) => socket,
            Err(e) => return Err(e),
        };

        Ok(Self { socket })
    }

    /// Receive a datagram message.
    pub fn recv(&self, buffer: &mut [u8]) -> io::Result<usize> {
        let res = self.socket.recv(buffer);
        
        let byte_count: usize = match res {
            Ok(count) => count,
            Err(e) => return Err(e),
        };

        let mut rng = rand::thread_rng();

        // Packet drop
        if rng.gen::<f64>() <= config::virtual_socket_errors::DROP_RATE {
            Self::log_socket_event("Packet dropped");
            // recursive call to return next incoming packet to simulate a packet actually getting
            // lost in the way.
            return self.recv(buffer);
        }

        Ok(byte_count)
    }

    fn log_socket_event(msg: &str) {
        println!("VIRTUAL SOCKET EVENT: {}", msg);
    } 
}


