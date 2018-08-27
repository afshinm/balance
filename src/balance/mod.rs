use backend::Backend;
use std::net::SocketAddr;

#[derive(Clone)]
pub struct Balance {
    pub socket_addr: SocketAddr,
    pub backends: Vec<Backend>
}

impl Balance {
    pub fn new(socket_addr: SocketAddr) -> Self {
        return Balance {
            socket_addr: socket_addr,
            backends: Vec::<Backend>::new()
        };
    }

    pub fn add_backend(&mut self, backend: Backend) {
        self.backends.push(backend);
    }

    /*
    pub fn next<'a>(self) -> &'a Backend {
        &self.backends[0]
    }*/
}
