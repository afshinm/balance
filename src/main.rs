extern crate futures;
extern crate tokio;
extern crate tokio_io;
#[macro_use]
extern crate clap;

use std::net::SocketAddr;
use clap::App;
use futures::{Future, Stream};
use tokio::executor::current_thread;
use tokio::net::TcpListener;
use tokio_io::{io, AsyncRead};

mod balance;
mod backend;

use balance::Balance;
use backend::Backend;

fn main() {
    let yaml = load_yaml!("cli.yaml");
    let matches = App::from_yaml(yaml).get_matches();

    let raw_backends: Vec<&str> = matches.values_of("worker").unwrap().collect();
    let port: u16 = matches.value_of("port").unwrap_or("8789").parse::<u16>().unwrap();

    println!("backends: {:?}", raw_backends);
    println!("port: {:?}", port);

    // Bind the server's socket
    let raw_addr = format!("127.0.0.1:{}", port);
    let addr = raw_addr.parse::<SocketAddr>().unwrap();

    let tcp = TcpListener::bind(&addr).unwrap();

    let balance = Balance::new(&addr);

    for raw_backend in raw_backends {
        //balance.add_backend(Backend::new(&raw_backend.parse::<SocketAddr>().unwrap()));
    }

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
        println!("Balance is running on port {}", port);
    });
}
