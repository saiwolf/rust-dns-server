#![allow(clippy::upper_case_acronyms)]
#![allow(clippy::unused_io_amount)]
pub mod byte_packet_buffer;
pub mod query_type;
pub mod result_code;
pub mod dns;

use std::net::UdpSocket;

use byte_packet_buffer::BytePacketBuffer;
use dns::packet::DnsPacket;
use dns::question::DnsQuestion;
use query_type::QueryType;

type Error = Box<dyn std::error::Error>;
type Result<T> = std::result::Result<T, Error>;



fn main() -> Result<()> {
    // Perform an A query for google.com
    let qname = "yahoo.com";
    let qtype = QueryType::MX;

    // Using google's public DNS
    let server = ("8.8.8.8", 53);

    // Bind an UDP socket to an arbitrary port
    let socket = UdpSocket::bind(("0.0.0.0", 43210))?;

    let mut packet = DnsPacket::new();
    
    packet.header.id = 6666;
    packet.header.questions = 1;
    packet.header.recursion_desired = true;
    packet
        .questions
        .push(DnsQuestion::new(qname.to_string(), qtype));
    
    // Use our new write method to write the packet to a buffer...
    let mut req_buffer = BytePacketBuffer::new();
    packet.write(&mut req_buffer).unwrap();

    // ... and send it off to the server using our socket:
    socket.send_to(&req_buffer.buf[0..req_buffer.pos], server)?;

    // To prepare for receiving the response, we'll create a new `BytePacketBuffer`,
    // and ask the socket to write the response directly into our buffer.
    let mut res_buffer = BytePacketBuffer::new();
    socket.recv_from(&mut res_buffer.buf)?;

    // As per the previous section, `DnsPacket::from_buffer()` is then used to
    // actually parse the packet after which we can print the response.
    let res_packet = DnsPacket::from_buffer(&mut res_buffer)?;
    println!("{:#?}", res_packet.header);

    for q in res_packet.questions {
        println!("{:#?}", q);
    }
    for rec in res_packet.answers {
        println!("{:#?}", rec);
    }
    for rec in res_packet.authorities {
        println!("{:#?}", rec);
    }
    for rec in res_packet.resources {
        println!("{:#?}", rec);
    }

    Ok(())
}
