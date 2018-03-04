use futures::{Future, Stream};
use tokio::net::TcpListener;
use tokio::executor::current_thread;
use tokio_io::{io, AsyncRead};
use Balance;

pub struct Listener;


impl Listener {
    pub fn start(balance: &Balance) {
        let tcp = TcpListener::bind(&balance.socket_addr).unwrap();

        // Iterate incoming connections
        let server = tcp.incoming().for_each(|tcp| {
            println!("connection");

            // Split up the read and write halves
            let (reader, writer) = tcp.split();

            // Copy the data back to the client
            let conn = io::copy(reader, writer)
                // print what happened
                .map(|(n, _, _)| {
                    println!("wrote {} bytes", n)
                })
            // Handle any errors
            .map_err(|err| {
                println!("IO error {:?}", err)
            });

            // Spawn the future as a concurrent task
            current_thread::spawn(conn);

            Ok(())
        })
        .map_err(|err| {
            println!("server error {:?}", err);
        });

        // Spin up the server on the event loop
        current_thread::run(|_| {
            current_thread::spawn(server);
            println!("Balance is running on {}", balance.socket_addr);
        });
    }
}

