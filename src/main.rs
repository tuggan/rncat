use std::net::{TcpListener, TcpStream};

use std::io::Read;
use std::io::Write;
use std::io::stdout;


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
        match stdout().write(&buf) {
            Err(e) => panic!("Got error while writing to stdout: {}", e),
            Ok(s) => println!("Wrote {} bytes", s),
        };
    }
}


fn main() {
    match TcpListener::bind("127.0.0.1:8888") {
        Err(e) => panic!("Error while binding to adress: {}", e),
        Ok(l) => {
            for stream in l.incoming() {
                match stream {
                    Err(e) => println!("failed: {}", e),
                    Ok(stream) => print_incomming(stream),
                }
            }
        }
    };
}
