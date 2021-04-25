#![allow(clippy::upper_case_acronyms)]
#![allow(clippy::unused_io_amount)]
pub mod byte_packet_buffer;
pub mod query_type;
pub mod result_code;
pub mod dns;

use std::fs::File;
use std::io::Read;

use byte_packet_buffer::BytePacketBuffer;
use dns::packet::DnsPacket;

type Error = Box<dyn std::error::Error>;
type Result<T> = std::result::Result<T, Error>;



fn main() -> Result<()> {
    let mut f = File::open("response_packet.txt")?;
    let mut buffer = BytePacketBuffer::new();
    f.read(&mut buffer.buf)?;

    let packet = DnsPacket::from_buffer(&mut buffer).unwrap();
    println!("{:#?}", packet.header);

    for q in packet.questions {
        println!("{:#?}", q);
    }
    for rec in packet.answers {
        println!("{:#?}", rec);
    }
    for rec in packet.authorities {
        println!("{:#?}", rec);
    }
    for rec in packet.resources {
        println!("{:#?}", rec);
    }

    Ok(())
}
