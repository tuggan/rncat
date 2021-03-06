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

extern crate getopts;

use getopts::Options;
use std::env;

mod listener;
mod writer;

fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} FILE [options]", program);
    print!("{}", opts.usage(&brief));
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();
    let mut listener: bool = false;
    let mut adress = String::from("0.0.0.0");
    let mut port = String::from("8888");

    let mut opts = Options::new();

    opts.optflag("l", "listen", "Listen on port ");
    opts.optflag("h", "help", "Print this help menu");
    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(e) => panic!("  {}", e),
    };

    if matches.opt_present("h") {
        print_usage(&program, opts);
        return;
    }

    if matches.opt_present("l") {
        listener = true;
    }

    if matches.free.len() == 2 {
        adress = matches.free[0].clone();
        port = matches.free[1].clone();
    };

    if listener {
        listener::listen(&adress, &port);
    } else {
        writer::write(&adress, &port);
    }
}
