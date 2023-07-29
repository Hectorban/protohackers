use std::io::{Read, Write};
use std::net::TcpStream;

fn main() {
    let mut stream = TcpStream::connect(":::4000").expect("Could not connect to server");

    let request = b"Hello server!";
    stream.write_all(request).expect("Failed to write to server");
    println!("Sending: {:?}", request);

    let mut buffer = [0; 13];
    stream.read(&mut buffer).expect("Failed to read from server");
    println!("Received: {:?}", &buffer[..]);
}
