// todo: how do I summarize the io imports (I'm assuming if I do `use std::io::{BufRead, BufReader, Write}`,
// I will overwrite the default Result?)
use std::io;
use std::io::BufRead;
use std::io::BufReader;
use std::net::{TcpListener, TcpStream};
use sixtop_rs::msg_reader::{deserialize_message};


const IP_AND_PORT: &str = "127.0.0.1:8080";

fn handle_client_connection(stream: TcpStream) -> Result<(), io::Error> {
    println!(
        "handling incoming connection from {:?}",
        stream.local_addr().unwrap()
    );

    let mut buffer = Vec::new();
    let mut stream_reader = BufReader::new(stream.try_clone().unwrap());

    // \n is a hacky way to read into vec instead of st bc I can*t be bothered rn
    stream_reader.read_until(b'\n', &mut buffer).unwrap();

    // return message
    println!("(/◔ ◡ ◔)/~ {:?}", buffer);

    let msg = deserialize_message(buffer).expect("unable to parse message");
    println!("{:?}", msg);

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
