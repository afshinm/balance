use std::net::SocketAddr;

#[derive(Clone)]
pub struct Backend<'a> {
    socket_addr: &'a SocketAddr,
}

impl<'a> Backend<'a> {
    pub fn new(socket_addr: &'a SocketAddr) -> Self {
        return Backend {
            socket_addr: socket_addr
        };
    }
}
