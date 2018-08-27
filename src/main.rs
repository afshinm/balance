extern crate futures;
extern crate tokio;
extern crate tokio_core;
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
mod listener;

use balance::Balance;
use backend::Backend;
use listener::Listener;

fn main() {
    let yaml = load_yaml!("cli.yaml");
    let matches = App::from_yaml(yaml).get_matches();

    let raw_backends: Vec<&str> = matches.values_of("worker").unwrap().collect();
    let port: u16 = matches.value_of("port").unwrap_or("8789").parse::<u16>().unwrap();

    let raw_addr = format!("127.0.0.1:{}", port);
    let addr = raw_addr.parse::<SocketAddr>().unwrap();

    let mut balance = Balance::new(addr);

    for raw_backend in raw_backends {
        let backend_addr = raw_backend.parse::<SocketAddr>().expect("Invalid backend address");
        balance.add_backend(Backend::new(backend_addr));
    }

    Listener::start(&balance);
}
