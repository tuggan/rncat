use std::net::{TcpListener, TcpStream};

use std::io::Read;
use std::io::Write;
use std::io::Stdout;


fn print_incomming(mut stream: TcpStream) {
    let mut buf;
    loop {
        buf = [0; 512];
        let _ = match stream.read(&mut buf) { 
            Err(e) => panic!("Got an error {}", e),
            Ok(m) => {
                if m == 0 {
                    break;
                }
                m
            }
        };
        Stdout::write(&mut std::io::stdout(), &buf);
    }
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8888").unwrap();
    for stream in listener.incoming() {
        match stream {
            Err(e) => println!("failed: {}", e),
            Ok(stream) => print_incomming(stream),
        }
    }
}
