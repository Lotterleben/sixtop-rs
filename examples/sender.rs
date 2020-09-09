extern crate sixtop_rs;

use std::io::Write;
use std::net::{TcpStream};

use sixtop_rs::types::{MsgHdr, MsgType,
                       Request, RequestType};
use sixtop_rs::msg_builder::{serialize_header};

const SERVER_ADDR: &str = "127.0.0.1:8080";

fn build_msg() -> Vec<u8> {
    let mut test_hdr = MsgHdr::new();
    test_hdr.msg_type = MsgType::REQUEST;
    test_hdr.code = RequestType::ADD as u8;
    test_hdr.seqnum = 22;

    let hdr = serialize_header(test_hdr).unwrap();
    return hdr;
}

fn main() {
    let mut stream = TcpStream::connect(SERVER_ADDR).unwrap();
    let msg = build_msg();
    match stream.write(msg.as_slice()) {
        Ok(numbytes) => {println!("wrote {} bytes", numbytes)}
        Err(error) => { println!("whoops. err {:?}", error)}
    }
}