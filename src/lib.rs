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

mod cmdline;
mod error;
mod pre;
mod types;

pub use cmdline::{open_istream, open_ostream, read_prog};
pub use error::{Error, Result};

use std::{
    io::{self, Read, Write},
    u8,
};
use types::{CharBuf, Strip};

/// All valid brainfuck operators.
const VALID_CHARS: [char; 8] = ['+', '-', '<', '>', '.', ',', '[', ']'];

#[derive(Default)]
pub struct Interpreter<'a> {
    bfin: Option<&'a mut Read>,
    bfout: Option<&'a mut Write>,
    addr_ptr: i64,
    prog_ctr: usize,
    skip_ctr: u32,
    strip: Strip,
    jumps: Vec<usize>,
    char_buf: CharBuf,
}

impl<'a> Interpreter<'a> {
    pub fn new() -> Self {
        Self {
            bfin: None,
            bfout: None,
            addr_ptr: 0,
            prog_ctr: 0,
            skip_ctr: 0,
            strip: Strip::new(),
            jumps: Vec::new(),
            char_buf: CharBuf::new(),
        }
    }

    pub fn bfin(mut self, ins: &'a mut impl Read) -> Self {
        self.bfin = Some(ins);
        self
    }

    pub fn bfout(mut self, outs: &'a mut impl Write) -> Self {
        self.bfout = Some(outs);
        self
    }

    /// Runs a brainfuck program.
    ///
    /// This function takes a brainfuck program in form of
    /// a string, preprocesses it and executes it in-memory.
    /// # Errors
    /// Returns any of the runtime errors in `bfrun::error::Error`.
    /// # Panics
    /// If an IO error occurs during the execution of the brainfuck program.
    pub fn run(&mut self, prog: &str) -> Result {
        let prog = pre::process(prog)?;
        let endval = prog.len();

        while self.prog_ctr < endval {
            let c = prog[self.prog_ctr];

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

            self.prog_ctr += 1; // Increment loop counter.
        }

        self.flush_buf();

        // Clear used fields.
        self.addr_ptr = 0;
        self.prog_ctr = 0;
        self.skip_ctr = 0;
        self.strip.clear();
        self.jumps.clear();
        self.char_buf.clear();

        Ok(())
    }

    fn exec(&mut self, c: char, num: u32) {
        match c {
            '+' => {
                let t = self.read().wrapping_add(num as u8);
                self.write(t);
            }
            '-' => {
                let t = self.read().wrapping_sub(num as u8);
                self.write(t);
            }
            '<' => self.addr_ptr -= i64::from(num),
            '>' => self.addr_ptr += i64::from(num),
            '.' => self.write_byte(),
            ',' => self.read_byte(),
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
        let mut buf = vec![0; 1];

        if let Some(s) = &mut self.bfin {
            s.read_exact(&mut buf)
        } else {
            io::stdin().read_exact(&mut buf)
        }
        .expect("error while reading from bfin");

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
        let buf = &[self.read(); 1];

        if let Some(s) = &mut self.bfout {
            s.write_all(buf)
        } else {
            io::stdout().write_all(buf)
        }
        .expect("error while writing to bfout");
    }

    fn flush_buf(&mut self) {
        if let Some(c) = self.char_buf.ch_opt {
            self.exec(c, self.char_buf.ctr)
        }

        self.char_buf.clear();
    }
}

#[cfg(test)]
mod test_runbf {
    use super::*;

    const HELLO_WORLD_PROG: &str = "[,.[.],..,,,+,-,<>,[]..]++++++++[>++++[>++>+++>+++>+<<<<-]>+>+>->>+[<]<-]>>.>---.+++++++..+++.>>.<-.<.+++.------.--------.>>+.>++.";

    #[test]
    fn runtime_error() {
        let prog = read_file("examples/hello_world.b").unwrap();
        Interpreter::new().run(prog).unwrap();
    }

    #[test]
    fn hello_world() {
        let prog: Vec<char> = HELLO_WORLD_PROG.chars().collect();
        let mut bfout = Vec::new();
        let expected = "Hello World!\n";
        Interpreter::new().bfout(&mut bfout).run(prog).unwrap();
        assert_eq!(&String::from_utf8(bfout).unwrap(), expected)
    }
}
