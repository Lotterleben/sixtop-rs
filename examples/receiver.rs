// todo: how do I summarize the io imports (I'm assuming if I do `use std::io::{BufRead, BufReader, Write}`,
// I will overwrite the default Result?)
use sixtop_rs::msg_builder::serialize_response;
use sixtop_rs::msg_reader::deserialize_message;
use sixtop_rs::types::SixtopMsg;
use std::io;
use std::io::{BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream};

const IP_AND_PORT: &str = "127.0.0.1:8080";

fn handle_client_connection(mut stream: TcpStream) -> Result<(), io::Error> {
    println!(
        "handling incoming connection from {:?}",
        stream.local_addr().unwrap()
    );

    let mut stream_reader = BufReader::new(stream.try_clone().unwrap());

    // todo loop here
    let buffer = stream_reader.fill_buf()?.to_vec();
    let msg = deserialize_message(buffer).expect("unable to parse message");
    println!("received: {:#?}", msg);

    let result = sixtop_rs::handle_msg(msg).unwrap();
    if let Some(response) = result {
        match response {
            SixtopMsg::ResponseMsg(response) => {
                println!("answered: {:#?}", response);
                let data = serialize_response(response).unwrap();
                stream.write(data.as_slice())?;
            }
            _ => unimplemented!(),
        }
    }
    Ok(())
}

fn main() -> io::Result<()> {
    // listen on 127.0.0.1:8080
    let listener = TcpListener::bind(IP_AND_PORT)?;
    println!("listening on {}", IP_AND_PORT);

    // accept connections
    for stream in listener.incoming() {
        println!("waiting for connection...");
        handle_client_connection(stream?)?;
    }

    Ok(())
}
