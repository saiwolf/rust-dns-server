#![allow(clippy::upper_case_acronyms)]
#![allow(clippy::unused_io_amount)]
#![allow(dead_code)]

pub mod primitives;
pub mod dns;
pub mod server;

use std::net::UdpSocket;

use server::handle_query;

type Error = Box<dyn std::error::Error>;
type Result<T> = std::result::Result<T, Error>;

fn main() -> Result<()> {
    // Bind an UDP socket on port 2053
    let socket = UdpSocket::bind(("0.0.0.0", 2053))?;

    // For now, queries are handled sequentially, so an infinite loop for servicing
    // requests is initiated.
    loop {
        match handle_query(&socket) {
            Ok(_) => {},
            Err(e) => eprintln!("An error occurred: {}", e),
        }
    }
}
