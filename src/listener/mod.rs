use futures::stream::Stream;
use futures::{Future, Poll};
use tokio_core::net::{TcpListener, TcpStream};
use tokio_core::reactor::Core;
use tokio_io::{AsyncRead, AsyncWrite};
use tokio_io::io::{copy, shutdown};

use Backend;
use balance::Balance;

use std::sync::Arc;
use std::env;
use std::net::{Shutdown, SocketAddr};
use std::io::{self, Read, Write};

pub struct Listener;

impl Listener {
    pub fn start(balance: &Balance) {
        // Create the event loop that will drive this server.
        let mut l = Core::new().unwrap();
        let handle = l.handle();

        let tcp = TcpListener::bind(&balance.socket_addr, &handle).unwrap();
        println!("Balance is running on {}", balance.socket_addr);

        // Iterate incoming connections
        let server_stream = tcp.incoming().for_each(move |(server, server_addr)| {
            println!("connection");

            let raw_addr = format!("127.0.0.1:{}", 80);
            let addr = raw_addr.parse::<SocketAddr>().unwrap();

            let backend_stream = TcpStream::connect(&addr, &handle);

            backend_stream.and_then(|backend| {
                println!("backend connected");

                let backend_writer = MyTcpStream(Arc::new(backend));
                let server_reader = MyTcpStream(Arc::new(server));

                copy(server_reader, backend_writer)
            });

            let msg = backend_stream.map(move |from_client| {
                println!("client at wrote bytes and received  bytes")
            }).map_err(|e| {
                // Don't panic. Maybe the client just disconnected too soon.
                println!("error: {}", e);
            });

            handle.spawn(msg);

            Ok(())
        })
        .map_err(|err| {
            println!("server error {:?}", err);
        });

        l.run(server_stream).unwrap();
    }
}

#[derive(Clone)]
struct MyTcpStream(Arc<TcpStream>);

impl Read for MyTcpStream {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        (&*self.0).read(buf)
    }
}

impl Write for MyTcpStream {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        (&*self.0).write(buf)
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

impl AsyncRead for MyTcpStream {}

impl AsyncWrite for MyTcpStream {
    fn shutdown(&mut self) -> Poll<(), io::Error> {
        try!(self.0.shutdown(Shutdown::Write));
        Ok(().into())
    }
}
