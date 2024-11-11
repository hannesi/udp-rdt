use std::net::{SocketAddr, UdpSocket};

use virtual_socket::VirtualSocket;

mod crc;
mod rdt_packet;
mod reliability_layer_with_error_detection;

pub trait RDTClient {
    fn new(socket: UdpSocket, dest_addr: SocketAddr) -> Self;
    fn send(&self, data: &[u8]);
}

pub trait RDTServer {
    fn new(socket: VirtualSocket) -> Self;
    fn recv(&self, buffer: &mut Vec<u8>) -> std::io::Result<usize>;
}

fn print_rdt_event(msg: String) {
    println!("RDT EVENT: {}", msg);
}
