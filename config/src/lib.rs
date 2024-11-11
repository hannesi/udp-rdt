pub mod server_info {
    pub const ADDRESS: std::net::Ipv4Addr = std::net::Ipv4Addr::new(127, 0, 0, 1);
    pub const PORT: u16 = 42069;
}

pub mod virtual_socket_errors {
    pub const BIT_ERROR_RATE: f64 = 0.25;
    pub const DELAY_RATE: f64 = 0.0;
    pub const DELAY_DURATION_MS: std::time::Duration = std::time::Duration::from_millis(2000);
    pub const DROP_RATE: f64 = 0.0;
}
