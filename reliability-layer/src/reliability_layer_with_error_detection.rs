use std::net::{SocketAddr, UdpSocket};

use virtual_socket::VirtualSocket;

use crate::{print_rdt_event, rdt_packet::RdtPacket, RDTClient, RDTServer};

struct Server {
    socket: VirtualSocket,
}

impl RDTServer for Server {
    fn new(socket: VirtualSocket) -> Self {
        Self { socket }
    }

    fn recv(&self, buffer: &mut Vec<u8>) -> Result<usize, std::io::Error> {
        let res = self.socket.recv(buffer);
        let res_size = match res {
            Ok(size) => size,
            Err(_) => todo!(),
        };
        let packet = match RdtPacket::try_from(buffer) {
            Ok(p) => p,
            Err(_) => todo!(),
        };

        if !packet.validate_crc_byte() {
            print_rdt_event("Bit error detected.".to_string());
        }

        Ok(res_size)
    }
}

// Client side handling not necessary for task 2.1
struct Client {
    socket: UdpSocket,
}

impl RDTClient for Client {
    fn new(socket: UdpSocket, dest_addr: SocketAddr) -> Self {
        todo!()
    }

    fn send(data: &[u8]) {
        todo!()
    }
}
