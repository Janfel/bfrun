//! A brainfuck interpreter written in Rust.
//!
//! This program aims to be a straightforward interpreter
//! for the brainfuck programming language. It is still
//! under development and breaking changes are to be expected.

/*
 * Copyright (C) 2019 Jan Felix Langenbach
 *
 * This file is part of bfrun.
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program.  If not, see <http: //www.gnu.org/licenses/>.
 */

mod analyze;
pub mod error;
pub use error::{Error, Result};
mod ops;
use std::{
    collections::HashMap,
    fs,
    io::{self, Read, Write},
    u8,
};

/// All valid brainfuck operators.
const VALID_CHARS: [char; 8] = ['+', '-', '<', '>', '.', ',', '[', ']'];

/// A struct for buffering chars.
///
/// Useful for counting chars before batch-executing them.
#[derive(Default)]
struct CharBuf {
    /// The char that is currently being buffered, if any.
    pub ch_opt: Option<char>,
    /// Counts the number of instances in the buffer.
    pub ctr: u32, // TODO Change everything to u32/i32.
}

impl CharBuf {
    pub fn new() -> Self {
        Self::default()
    }

    /// Inserts a char into the buffer and increments `ctr`.
    ///
    /// # Panics
    /// If the buffer currently contains a char other than `c`.
    pub fn insert(&mut self, c: char) {
        match self.ch_opt {
            Some(val) if val == c => self.ctr += 1,
            Some(_) => panic!("inserted new char into non-empty CharBuf"),
            None => {
                self.ch_opt = Some(c);
                self.ctr = 1
            }
        }
    }

    /// Resets the buffer to an empty state.
    pub fn clear(&mut self) {
        self.ch_opt = None;
        self.ctr = 0;
    }

    // TODO Change to callback fn.
    pub fn flush(&mut self, mut strip: &mut Strip, addr_ptr: &mut i64) {
        if self.ch_opt.is_none() {
            self.clear(); // Setting self.ctr = 0 would do the trick, but also introduce duplication.
            return;
        }

        // TODO Extract exec() fn.
        match self.ch_opt.unwrap() {
            '+' => {
                let t = ops::get(&mut strip, *addr_ptr).wrapping_add(ops::trunc(self.ctr));
                strip.insert(*addr_ptr, t);
            }
            '-' => {
                let t = ops::get(&mut strip, *addr_ptr).wrapping_sub(ops::trunc(self.ctr));
                strip.insert(*addr_ptr, t);
            }
            '<' => *addr_ptr -= self.ctr as i64, // !! Beware cast errors.
            '>' => *addr_ptr += self.ctr as i64, // TODO Use crate `cast`.
            _ => {}
        };
        self.clear();
    }
}

pub struct Interpreter<'a> {
    bfin: &'a mut Read,
    bfout: &'a mut Write,
    strip: Strip,
    jumps: Vec<usize>,
    addr_ptr: i64,
    skip_ctr: u32,
    char_buf: CharBuf,
    dirty: bool,
}

impl<'a> Interpreter<'a> {
    pub fn new(bfin: &'a mut impl Read, bfout: &'a mut impl Write) -> Self {
        Self {
            bfin,
            bfout,
            strip: Strip::new(),
            jumps: Vec::new(),
            addr_ptr: 0,
            skip_ctr: 0,
            char_buf: CharBuf::new(),
            dirty: false,
        }
    }

    pub fn clear(&mut self) {
        self.strip = Strip::new();
        self.jumps = Vec::new();
        self.addr_ptr = 0;
        self.skip_ctr = 0;
        self.char_buf = CharBuf::default();
    }

    /// Runs a brainfuck program.
    ///
    /// This function takes a slice of brainfuck instructions,
    /// preprocesses them and executes them in-memory.
    /// # Errors
    /// Returns any of the runtime errors in `bfrun::error::Error`.
    /// # Panics
    /// If an IO error occurs during the execution of the brainfuck program.
    pub fn run(&mut self, prog: &[char]) -> Result {
        analyze::all(prog)?;

        if self.dirty {
            self.clear();
            self.dirty = true;
        }

        let mut i = 0; // Loop counter.

        // TODO Change back to while {}.
        loop {
            // Check loop counter.
            if i >= prog.len() {
                self.char_buf.flush(&mut self.strip, &mut self.addr_ptr); // TODO Move after while {}.
                break;
            }

            let c = prog[i];

            if self.skip_ctr != 0 {
                match c {
                    '[' => self.skip_ctr += 1,
                    ']' => self.skip_ctr -= 1,
                    _ => {}
                }
                i += 1; // Increment loop counter.
                continue;
            }

            if let Some(buf_c) = self.char_buf.ch_opt {
                if buf_c != c {
                    self.char_buf.flush(&mut self.strip, &mut self.addr_ptr)
                }
            }

            // TODO Extract exec() fn.
            match c {
                '+' => self.char_buf.insert(c),
                '-' => self.char_buf.insert(c),
                '<' => self.char_buf.insert(c),
                '>' => self.char_buf.insert(c),
                '.' => ops::put_byte(ops::get(&mut self.strip, self.addr_ptr)),
                ',' => {
                    self.strip.insert(self.addr_ptr, ops::get_byte());
                }
                '[' => {
                    if ops::get(&mut self.strip, self.addr_ptr) == 0 {
                        self.skip_ctr = 1
                    } else {
                        self.jumps.push(i)
                    };
                }
                ']' => {
                    if ops::get(&mut self.strip, self.addr_ptr) == 0 {
                        self.jumps.pop();
                    } else {
                        i = *self.jumps.last().ok_or(Error::MissingLeftBracket)? as usize; // !! Beware casting errors
                    };
                }
                _ => (),
            };

            i += 1; // Increment loop counter.
        }

        Ok(())
    }
}

// TODO Integrate into preprocessor.
pub fn read_file(fname: &str) -> io::Result<Vec<char>> {
    let prog = fs::read_to_string(fname)?
        .chars()
        .filter(|x| VALID_CHARS.contains(x))
        .collect();
    Ok(prog)
}

/// The strip of memory brainfuck uses.
type Strip = HashMap<i64, u8>;

#[cfg(test)]
mod test_runbf {
    use super::{read_file, Interpreter};
    use std::io;

    #[test]
    fn test_runtime_error() {
        let prog = read_file("examples/hello_world.b").unwrap();
        let mut bfin = io::stdin();
        let mut bfout = io::stdout();
        Interpreter::new(&mut bfin, &mut bfout).run(&prog).unwrap();
    }
}
