extern crate sixtop_rs;
use std::io::BufRead;
use std::io::BufReader;

use std::io::Write;
use std::net::TcpStream;

use sixtop_rs::msg_builder::serialize_request;
use sixtop_rs::msg_reader::deserialize_message;
use sixtop_rs::types::{Cell, Msg, Request, RequestType};
use sixtop_rs::Sixtop;

const SERVER_ADDR: &str = "127.0.0.1:8080";

fn build_msg() -> Vec<u8> {
    let mut test_msg = Request::new();
    test_msg.header.code = RequestType::ADD as u8;
    test_msg.header.seqnum = 22;

    test_msg.metadata = 0b1111_1111_0000_0000;
    test_msg.cell_options = 0b100;
    test_msg.num_cells = 2;
    test_msg.cell_list.push(Cell {
        slot_offset: 1,
        channel_offset: 2,
    });
    test_msg.cell_list.push(Cell {
        slot_offset: 3,
        channel_offset: 9,
    });
    test_msg.cell_list.push(Cell {
        slot_offset: 5,
        channel_offset: 2,
    });

    println!("Built msg: {:#?}", test_msg);
    let data = serialize_request(test_msg).unwrap();
    return data;
}

fn main() {
    let mut stream = TcpStream::connect(SERVER_ADDR).unwrap();
    let mut stream_reader = BufReader::new(stream.try_clone().unwrap());
    let mut sixtop = Sixtop::new();

    // send dummy request
    let request = build_msg();
    match stream.write(request.as_slice()) {
        Ok(numbytes) => println!("wrote {} bytes", numbytes),
        Err(error) => println!("whoops. err {:?}", error),
    }
    stream.flush().unwrap();

    // receive and parse answer
    let buffer = stream_reader.fill_buf().unwrap().to_vec();
    let response = deserialize_message(buffer).expect("unable to parse message");
    println!("received: {:#?}", response);

    let result = sixtop.handle_msg(43, response).unwrap();
}
