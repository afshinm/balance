use std::net::SocketAddr;

#[derive(Clone)]
pub struct Backend {
    socket_addr: SocketAddr,
}

impl Backend {
    pub fn new(socket_addr: SocketAddr) -> Self {
        return Backend {
            socket_addr: socket_addr
        };
    }
}
