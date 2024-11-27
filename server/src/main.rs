use std::net::SocketAddr;

use virtual_socket::VirtualSocket;

fn main() {
    println!("SERVER");

    // Create a socket
    let addr_string: String = format!(
        "{}:{}",
        config::server_info::ADDRESS,
        config::server_info::PORT
    );
    let socket_addr: SocketAddr = addr_string
        .parse::<SocketAddr>()
        .expect("Invalid socket address.");
    let socket: VirtualSocket = VirtualSocket::bind(socket_addr).expect("Failed to create virtual socket.");

    println!(
        "Listening to messages on port {}.",
        config::server_info::PORT
    );

    // listen for messages
    loop {
        // buffer vec has to be initiated or it won't work with udp recv
        let mut buffer: Vec<u8> = vec![0; 1024];
        let res = socket.recv(&mut buffer);
        let byte_count: usize = match res {
            Ok(count) => count,
            Err(_) => continue,
        };

        let msg_string = String::from_utf8_lossy(&buffer[..byte_count]);
        println!("{}", msg_string);
    }
}
