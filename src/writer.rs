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
