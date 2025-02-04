use std::{net::{SocketAddr, UdpSocket}, thread};

use rand::Rng;

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
    pub fn recv(&self, buffer: &mut [u8]) -> std::io::Result<usize> {
        let res = self.socket.recv_from(buffer);

        let byte_count: usize = match res {
            Ok(content) => content.0,
            Err(e) => return Err(e),
        };

        let mut rng = rand::thread_rng();

        // Drop a packet
        if rng.gen::<f64>() <= config::virtual_socket_errors::DROP_RATE {
            Self::log_socket_event("Packet dropped");
            // recursive call to return next incoming packet to simulate a packet actually getting
            // lost in the way.
            return self.recv(buffer);
        }

        // Delay a packet
        if rng.gen::<f64>() <= config::virtual_socket_errors::DELAY_RATE {
            Self::log_socket_event("Packet delayed");
            thread::sleep(config::virtual_socket_errors::DELAY_DURATION_MS);
        }

        // Introduce a bit error
        let packet_bit_error_rng: f64 = rng.gen();
        if byte_count > 0 && packet_bit_error_rng <= config::virtual_socket_errors::BIT_ERROR_RATE {
            Self::log_socket_event("Bit error introduced");
            let byte_index = rng.gen_range(0..byte_count);
            let bit_index = rng.gen_range(0..8);
            // introduce bit error
            // XOR the selected byte with a mask that has the selected bit set to 1
            buffer[byte_index] ^= 1 << bit_index;
        }


        Ok(byte_count)
    }

    fn log_socket_event(msg: &str) {
        println!("VIRTUAL SOCKET EVENT: {}", msg);
    }
}
