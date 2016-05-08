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


extern crate rand;
use self::rand::Rng;


pub struct BoardBuilder {
    startpos: u32,
    cells_str: String
}

impl BoardBuilder {
    pub fn new() -> BoardBuilder {
        BoardBuilder {
            startpos: 0,
            cells_str: String::new() // no data yet
        }
    }
    pub fn cells(&mut self, data: &str) -> &mut BoardBuilder {
        self.cells_str = data.to_string();
        self
    }
    pub fn startpos(&mut self, startpos: u32) -> &mut BoardBuilder {
        self.startpos = startpos;
        self
    }
    pub fn build(&mut self) -> Board {
        if self.cells_str.is_empty() {
            // generate random data (default size: 14*14 cells; 6 colors)
            self.cells_str = rand::thread_rng()
                                    .gen_ascii_chars()
                                    .filter(|c| (c >= &'1') && (c <= &'6'))
                                    .take(14*14)
                                    .collect();
        }
        let sq = (self.cells_str.len() as f64).sqrt() as u32;
        let data: Vec<u8> = self.cells_str
                                    .chars()
                                    .map( |c|   if (c >= '1') && (c <= '9')
                                                { (c as u8) - ('1' as u8) } // subtract one!
                                                else { 0u8 } )
                                    .collect();
        let colors = (*data.iter().max().unwrap_or(&0) as u32) + 1; // add one!
        // TODO a better check if we have cells data for a square board
        debug_assert_eq!(sq*sq, data.len() as u32);
        Board {
            width: sq,
            height: sq,
            startpos: self.startpos,
            cells: data,
            colors: colors
        }
    }
}

#[derive(Debug)]
pub struct Board {
    width: u32,
    height: u32,
    startpos: u32,
    cells: Vec<u8>,
    colors: u32,
}
