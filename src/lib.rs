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
use std::{
    collections::HashMap,
    fs,
    io::{self, Read, Write},
    u8,
};

const LOGGING: bool = true;

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
}

pub struct Interpreter<'a> {
    bfin: Option<&'a mut Read>,
    bfout: Option<&'a mut Write>,
    strip: Strip,
    jumps: Vec<usize>,
    addr_ptr: i64,
    prog_ctr: usize,
    skip_ctr: u32,
    char_buf: CharBuf,
}

impl<'a> Interpreter<'a> {
    pub fn new() -> Self {
        Self {
            bfin: None,
            bfout: None,
            strip: Strip::new(),
            jumps: Vec::new(),
            addr_ptr: 0,
            prog_ctr: 0,
            skip_ctr: 0,
            char_buf: CharBuf::new(),
        }
    }
    // TODO Add bfin bfout builders
    pub fn clear(&mut self) {
        self.strip = Strip::new();
        self.jumps = Vec::new();
        self.addr_ptr = 0;
        self.prog_ctr = 0;
        self.skip_ctr = 0;
        self.char_buf = CharBuf::default();
        if LOGGING {
            eprintln!("I'm clear")
        }
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

        let endval = prog.len();

        // TODO Change back to while {}.
        while self.prog_ctr < endval {
            let c = prog[self.prog_ctr];

            if LOGGING {
                eprintln!(
                    "Loop {} Char {} Ptr {} Strip {:?}",
                    self.prog_ctr, c, self.addr_ptr, self.strip
                )
            }

            if self.skip_ctr != 0 {
                match c {
                    '[' => self.skip_ctr += 1,
                    ']' => self.skip_ctr -= 1,
                    _ => {}
                }
                self.prog_ctr += 1; // Increment loop counter.
                continue;
            }

            if let Some(buf_c) = self.char_buf.ch_opt {
                if buf_c != c {
                    self.flush_buf();
                }
            }

            match c {
                '+' => self.char_buf.insert(c),
                '-' => self.char_buf.insert(c),
                '<' => self.char_buf.insert(c),
                '>' => self.char_buf.insert(c),
                _ => self.exec(c, 1),
            };

            if LOGGING {
                eprintln!("While state {} of {}", self.prog_ctr, endval)
            }

            self.prog_ctr += 1; // Increment loop counter.
        }

        self.flush_buf();
        self.clear();

        if LOGGING {
            eprintln!("I'm through")
        }

        Ok(())
    }

    fn exec(&mut self, c: char, num: u32) {
        if LOGGING {
            eprintln!("Exec {} * {}", c, num)
        }
        match c {
            '+' => {
                let t = self.read().wrapping_add(trunc(num));
                self.write(t);
            }
            '-' => {
                let t = self.read().wrapping_sub(trunc(num));
                self.write(t);
            }
            '<' => self.addr_ptr -= i64::from(num),
            '>' => self.addr_ptr += i64::from(num),
            '.' => self.read_byte(),
            ',' => self.write_byte(),
            '[' => {
                if self.read() == 0 {
                    self.skip_ctr = 1
                } else {
                    self.jumps.push(self.prog_ctr)
                };
            }
            ']' => {
                if self.read() == 0 {
                    self.jumps.pop();
                } else {
                    self.prog_ctr = *self
                        .jumps
                        .last()
                        .ok_or(Error::MissingLeftBracket)
                        .expect("unsanitized code executed");
                };
            }
            _ => panic!("trying to execute invalid command char"),
        };
    }

    /// Reads out the active cell.
    ///
    /// Returns the value of the specified cell and
    /// initializes it with 0 if necessary.
    fn read(&mut self) -> u8 {
        *self.strip.entry(self.addr_ptr).or_insert(0)
    }

    /// Writes `val` into the active cell.
    fn write(&mut self, val: u8) {
        self.strip.insert(self.addr_ptr, val);
    }

    /// Reads one byte from `io::stdin` and saves it.
    ///
    /// # Panics
    /// If an `io::Error` occurs during `read_exact()`.
    /// This is a runtime error in the brainfuck program.
    /// A panic is justified because brainfuck has no
    /// means of handling such an error.
    fn read_byte(&mut self) {
        // TODO change to `impl Read`.
        let mut buf = vec![0; 1];

        if let Some(s) = &mut self.bfin {
            s.read_exact(&mut buf)
                .expect("error while reading from bfin"); // TODO Better error handling. Maybe bferr?
        } else {
            io::stdin()
                .read_exact(&mut buf)
                .expect("error while reading from bfin"); // TODO Better error handling. Maybe bferr?
        };

        self.write(buf[0]);
    }

    /// Writes the active cell as byte to `io::stdout`.
    ///
    /// # Panics
    /// If an `io::Error` occurs during `write_all()`.
    /// This is a runtime error in the brainfuck program.
    /// A panic is justified because brainfuck has no
    /// means of handling such an error.
    fn write_byte(&mut self) {
        let b = self.read();

        if let Some(s) = &mut self.bfout {
            s.write_all(&[b; 1]).expect("error while writing to bfout"); // TODO Better error handling. Maybe bferr?
        } else {
            io::stdout()
                .write_all(&[b; 1])
                .expect("error while writing to bfout"); // TODO Better error handling. Maybe bferr?
        }
    }

    fn flush_buf(&mut self) {
        if let Some(c) = self.char_buf.ch_opt {
            self.exec(c, self.char_buf.ctr)
        }

        self.char_buf.clear();

        if LOGGING {
            eprintln!("I'm flush")
        }
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

// TODO Test replacement by `x as u8`.
fn trunc(mut v: u32) -> u8 {
    let umax = u32::from(u8::MAX);
    while v > umax {
        v -= umax
    }
    v as u8
}

/// The strip of memory brainfuck uses.
type Strip = HashMap<i64, u8>;

#[cfg(test)]
mod test_runbf {
    use super::{read_file, Interpreter};

    #[test]
    fn test_runtime_error() {
        let prog = read_file("examples/hello_world.b").unwrap();
        Interpreter::new().run(&prog).unwrap();
    }
}
