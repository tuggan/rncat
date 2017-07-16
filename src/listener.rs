/*
   Copyright 2017 Dennis Vesterlund

   Licensed under the Apache License, Version 2.0 (the "License");
   you may not use this file except in compliance with the License.
   You may obtain a copy of the License at

       http://www.apache.org/licenses/LICENSE-2.0

   Unless required by applicable law or agreed to in writing, software
   distributed under the License is distributed on an "AS IS" BASIS,
   WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
   See the License for the specific language governing permissions and
   limitations under the License.
*/

use std::net::{TcpStream, TcpListener};

use std::io::Read;
use std::io::Write;
use std::io::stdout;

pub fn print_incomming(mut stream: TcpStream) {
    let mut buf;
    loop {
        // @TODO i only want to perform this operation once if possible
        buf = [0; 512];
        let _ = match stream.read(&mut buf) { 
            Err(e) => panic!("Failed to read stream: {}", e),
            Ok(m) => {
                if m == 0 {
                    break;
                }
                m
            }
        };
        match stdout().write(&buf) {
            Err(e) => panic!("Failed while writing to stdout: {}", e),
            Ok(s) => s,
        };
    }
}

pub fn listen(adress: &str, port: &str) {
    match TcpListener::bind(format!("{}:{}", adress, port)) {
        Err(e) => panic!("Error while binding to adress {}:{}: {}", adress, port, e),
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
