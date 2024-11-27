use core::str;
use std::net::{SocketAddr, UdpSocket};

fn main() {
    println!("CLIENT");

    // form destination address
    let dest_addr_string: String = format!(
        "{}:{}",
        config::server_info::ADDRESS,
        config::server_info::PORT
    );
    let dest_socket_addr: SocketAddr = dest_addr_string
        .parse::<SocketAddr>()
        .expect("Invalid destination address.");

    // create socket for sending data
    let socket_addr: SocketAddr = "127.0.0.1:0"
        .parse::<SocketAddr>()
        .expect("Invalid socket address.");
    let socket: UdpSocket = UdpSocket::bind(socket_addr).expect("Failed to create UDP socket.");

    println!("Ready to send messages to {}", dest_addr_string);
    println!("Usage: Type a message and hit enter :)");


    loop {
        let mut msg: String = String::new();
        let user_input = std::io::stdin().read_line(&mut msg);
        match user_input {
            Ok(_) => {
                let trimmed_msg = msg.trim();
                println!("Sending \"{}\" to {}", trimmed_msg, dest_addr_string);
                let buffer: &[u8] = trimmed_msg.as_bytes();
                dbg!(buffer);
                let res = socket.send_to(buffer, dest_socket_addr);
                match res {
                    Ok(_) => continue,
                    Err(err) => println!("Failed to send the message:\n{}", err),
                }
            }
            Err(_) => {
                println!("Invalid input. Try again :)");
                continue;
            }
        }
    }
}
