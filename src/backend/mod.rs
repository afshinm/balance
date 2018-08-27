use tokio_core::reactor::Core;
use tokio_core::net::TcpStream;
use tokio_core::net::TcpStreamNew;
use std::net::SocketAddr;
use std::sync::Arc;

#[derive(Clone)]
pub struct Backend {
    pub socket_addr: SocketAddr,
    pub connection: Arc<TcpStreamNew>
}

impl Backend {
    pub fn new(socket_addr: SocketAddr) -> Self {
        return Backend {
            socket_addr: socket_addr,
            connection: Arc::new(Self::open(&socket_addr))
        };
    }

    fn open(socket_addr: &SocketAddr) -> TcpStreamNew {
        let mut core = Core::new().unwrap();
        let handle = core.handle();

        TcpStream::connect(socket_addr, &handle)
    }
}
