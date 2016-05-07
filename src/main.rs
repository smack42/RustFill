// RustFill - a Flood-It solver written in Rust
// Copyright (C) 2016 Michael Henke
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <http://www.gnu.org/licenses/>.


use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

const VERSION: Option<&'static str> = option_env!("CARGO_PKG_VERSION");

fn main() {
    println!("RustFill v{}", VERSION.unwrap_or("_unknown"));
    match do_pc19() {
        Ok(_)  => { println!("done.") },
        Err(e) => { println!("error!"); println!("{}", e); std::process::exit(1); }
    }
}


fn do_pc19() -> std::io::Result<String> {
    let f = try!(File::open("pc19/tiles.txt"));
    let reader = BufReader::new(f);
    let mut num_lines = 0u32;
    // read and count all lines from the file
    for line in reader.lines() {
        let line: String = match line {
            Ok(s) => {
                // stop processing when line has unexpected length
                if s.len() != 14*14 { break; }
                else { num_lines += 1; s }
            },
            // return the error immediately
            Err(_) => { return line; }
        };
        println!("{:04} {}", num_lines, line);
    }
    println!("number of lines processed: {}", num_lines);
    Ok(String::new())
}
