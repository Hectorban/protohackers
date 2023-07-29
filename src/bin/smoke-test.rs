use std::io::{prelude::*, BufReader};
use std::net::{TcpListener, TcpStream};
use std::thread;

fn main() {
    let listener = TcpListener::bind(":::4000").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        thread::spawn(|| {
            handle_connection(stream);
        });
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buf_reader = BufReader::new(&mut stream);
    let mut return_buffer:Vec<u8> = Vec::new();
    let received = buf_reader.read_to_end(&mut return_buffer);
    match received {
        Ok(_) => {
            buf_reader.consume(return_buffer.len());
            println!("ECHOING BYTES: {:?}", return_buffer);
            stream.write(&return_buffer).unwrap();
        },
        Err(err) => { println!("Error: {}", err )}
    }
}
