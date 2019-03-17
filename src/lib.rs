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
use std::{collections::HashMap, fs, io, u8};

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
    pub ctr: usize, // TODO Change everything to u32/i32.
}

impl CharBuf {
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
    pub fn flush(&mut self, mut strip: &mut Strip, addr_ptr: &mut isize) {
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
            '<' => *addr_ptr -= self.ctr as isize, // !! Beware cast errors.
            '>' => *addr_ptr += self.ctr as isize, // TODO Use crate `cast`.
            _ => {}
        };
        self.clear();
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

/// Runs a brainfuck program.
///
/// This function takes a slice of brainfuck instructions,
/// preprocesses them and executes them in-memory.
/// # Errors
/// Returns any of the runtime errors in `bfrun::error::Error`.
/// # Panics
/// If an IO error occurs during the execution of the brainfuck program.
pub fn run(prog: &[char]) -> Result {
    analyze::all(prog)?;

    let mut strip = Strip::new();
    let mut jumps = Vec::new();
    let mut addr_ptr = 0;
    let mut skip_ctr = 0;
    let mut char_buf = CharBuf::default();

    let mut i = 0; // Loop counter.

    // TODO Change back to while {}.
    loop {
        // Check loop counter.
        if i >= prog.len() {
            char_buf.flush(&mut strip, &mut addr_ptr); // TODO Move after while {}.
            break;
        }

        let c = prog[i];

        if skip_ctr != 0 {
            match c {
                '[' => skip_ctr += 1,
                ']' => skip_ctr -= 1,
                _ => {}
            }
            i += 1; // Increment loop counter.
            continue;
        }

        if let Some(buf_c) = char_buf.ch_opt {
            if buf_c != c {
                char_buf.flush(&mut strip, &mut addr_ptr)
            }
        }

        // TODO Extract exec() fn.
        match c {
            '+' => char_buf.insert(c),
            '-' => char_buf.insert(c),
            '<' => char_buf.insert(c),
            '>' => char_buf.insert(c),
            '.' => ops::put_byte(ops::get(&mut strip, addr_ptr)),
            ',' => {
                strip.insert(addr_ptr, ops::get_byte());
            }
            '[' => {
                if ops::get(&mut strip, addr_ptr) == 0 {
                    skip_ctr = 1
                } else {
                    jumps.push(i)
                };
            }
            ']' => {
                if ops::get(&mut strip, addr_ptr) == 0 {
                    jumps.pop();
                } else {
                    i = *jumps.last().ok_or(Error::MissingLeftBracket)?;
                };
            }
            _ => (),
        };

        i += 1; // Increment loop counter.
    }

    Ok(())
}

/// The strip of memory brainfuck uses.
type Strip = HashMap<isize, u8>;

#[cfg(test)]
mod test_runbf {
    use super::{read_file, run};

    #[test]
    fn test_runtime_error() {
        let prog = read_file("hello_world.b").unwrap();
        run(&prog).unwrap();
    }
}
