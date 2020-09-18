extern crate sixtop_rs;

use std::io::Write;
use std::net::{TcpStream};

use sixtop_rs::types::{MsgType, Msg,
                       Cell,
                       Request, RequestType};
use sixtop_rs::msg_builder::{serialize_request};

const SERVER_ADDR: &str = "127.0.0.1:8080";

fn build_msg() -> Vec<u8> {
    let mut test_msg = Request::new();
    test_msg.header.msg_type = MsgType::REQUEST;
    test_msg.header.code = RequestType::ADD as u8;
    test_msg.header.seqnum = 22;

    test_msg.metadata = 0b1111_1111_0000_0000;
    test_msg.cell_options = 0b100;
    test_msg.num_cells = 3;
    test_msg.cell_list.push(Cell{slot_offset: 1, channel_offset: 2});
    test_msg.cell_list.push(Cell{slot_offset: 3, channel_offset: 9});

    println!("Built msg: {:?}", test_msg);
    let data = serialize_request(test_msg).unwrap();
    return data;
}

fn main() {
    let mut stream = TcpStream::connect(SERVER_ADDR).unwrap();
    let data = build_msg();
    match stream.write(data.as_slice()) {
        Ok(numbytes) => {
            println!("wrote {} bytes", numbytes)
        }
        Err(error) => { println!("whoops. err {:?}", error)}
    }
}