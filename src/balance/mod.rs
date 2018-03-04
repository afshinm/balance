use backend::Backend;
use std::net::SocketAddr;

#[derive(Clone)]
pub struct Balance<'a> {
    pub socket_addr: &'a SocketAddr,
    pub backends: Vec<Backend<'a>>
}

impl<'a> Balance<'a> {
    pub fn new(socket_addr: &'a SocketAddr) -> Self {
        return Balance {
            socket_addr: socket_addr,
            backends: Vec::<Backend>::new()
        };
    }

    pub fn add_backend(mut self, backend: Backend<'a>) {
        self.backends.push(backend);
    }
}
