//! Utility functions used throughout the program.

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

use super::Strip;
use std::{
    io::{self, Read, Write},
    u8,
};

// TODO Test replacement by `x as u8`.
pub fn trunc(mut v: u32) -> u8 {
    let umax = u8::MAX as usize;
    while v > umax {
        v -= umax
    }
    v as u8
}

/// Reads the cell with `index` from `strip`.
///
/// Returns the value of the specified cell and
/// initializes it with 0 if necessary.
pub fn get(strip: &mut Strip, index: i64) -> u8 {
    *strip.entry(index).or_insert(0)
}

/// Reads exactly one byte from `io::stdin`.
///
/// # Panics
/// If an `io::Error` occurs during `read_exact()`.
/// This is a runtime error in the brainfuck program.
/// A panic is justified because brainfuck has no
/// means of handling such an error.
pub fn get_byte() -> u8 {
    // TODO change to `impl Read`.
    let mut buf = vec![0; 1];
    io::stdin().read_exact(&mut buf).unwrap(); // TODO Better error handling.
    buf[0]
}

/// Writes exactly one byte to `io::stdout`.
///
/// # Panics
/// If an `io::Error` occurs during `write_all()`.
/// This is a runtime error in the brainfuck program.
/// A panic is justified because brainfuck has no
/// means of handling such an error.
pub fn put_byte(b: u8) {
    // TODO change to `impl Write`.
    io::stdout().write_all(&[b; 1]).unwrap(); // TODO Better error handling.
}
