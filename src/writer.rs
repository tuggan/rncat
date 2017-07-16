use std::io::Read;
use std::io::Write;
use std::io::stdin;

use std::net::TcpStream;

pub fn write(adress: &str, port: &str) {
    let mut buf;

    match TcpStream::connect(format!("{}:{}", adress, port)) {
        Err(e) => {
            panic!("Could not connect to adress {}:{}, error: {}",
                   adress,
                   port,
                   e)
        }
        Ok(mut stream) => {

            loop {
                // @TODO i only want to perform this operation once if possible
                buf = [0; 512];
                match stdin().read(&mut buf) {
                    Err(e) => panic!("Failed while reading from stdin: {}", e),
                    Ok(s) => {
                        if s == 0 {
                            break;
                        };
                        match stream.write(&buf) {
                            Err(e) => panic!("Failed while writing to stdout: {}", e),
                            Ok(s) => s,
                        };
                    }
                }
            }
        }
    }
}
