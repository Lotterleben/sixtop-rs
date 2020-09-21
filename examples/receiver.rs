// todo: how do I summarize the io imports (I'm assuming if I do `use std::io::{BufRead, BufReader, Write}`,
// I will overwrite the default Result?)
use std::io;
use std::io::BufRead;
use std::io::BufReader;
use std::io::Write;
use std::net::{TcpListener, TcpStream};
use sixtop_rs::types::{Msg, Response, ReturnCode, SixtopMsg};
use sixtop_rs::msg_reader::{deserialize_message};
use sixtop_rs::msg_builder::{serialize_response};

const IP_AND_PORT: &str = "127.0.0.1:8080";

// dummy handling for now
// returns an answer to be sent if necessary
fn dummy_handle_msg(msg: SixtopMsg) -> Result<Option<SixtopMsg>, ()>{
    match msg {
        SixtopMsg::RequestMsg(request) => {
            // dummy: pick first two cells and accept
            let mut response = Response::new();
            response.header.code = ReturnCode::RC_SUCCESS as u8;
            response.header.seqnum = request.header.seqnum; // TODO store and handle seqnum etc

            // just choose the first two cells. obvs missing coherence check etc
            for index in 0..request.num_cells {
                response.cell_list.push(*request.cell_list.get(index as usize).unwrap());
            }

            Ok(Some(SixtopMsg::ResponseMsg(response)))
        },
        SixtopMsg::ResponseMsg(response) => { unimplemented!() },
        _ => { unimplemented!() }
    }
}

fn handle_client_connection(mut stream: TcpStream) -> Result<(), io::Error> {
    println!(
        "handling incoming connection from {:?}",
        stream.local_addr().unwrap()
    );

    let mut buffer = Vec::new();
    let mut stream_reader = BufReader::new(stream.try_clone().unwrap());

    // todo loop here
    // \n is a hacky way to read into vec instead of st bc I can*t be bothered rn
    stream_reader.read_until(b'\n', &mut buffer).unwrap();

    let msg = deserialize_message(buffer).expect("unable to parse message");
    println!("received: {:#?}", msg);
    let result = dummy_handle_msg(msg).unwrap();
    if let Some(response) = result {
        match response {
            SixtopMsg::ResponseMsg(response) => {
                println!("answered: {:#?}", response);
                let data = serialize_response(response).unwrap();
                stream.write(data.as_slice());
             },
            _ => { unimplemented!() }
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
