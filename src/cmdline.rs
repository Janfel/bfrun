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

use std::{
    fs::{self, File, OpenOptions},
    io::{self, BufReader, BufWriter, Read, Write},
};

// TODO Proper error handling.

/// Reads the specified source into a string.
///
/// Opens the source, or `stdin` if `from` is `"-"`,
/// and reads it into a string until the first EOF.
/// # Panics
/// An error occurred while reading. This was decided
/// to be able to easily map this function over an iterator.
pub fn read_prog(from: &str) -> String {
    match from {
        "-" => {
            let mut buf = String::new();
            io::stdin()
                .read_to_string(&mut buf)
                .expect("unable to read from stdin");
            buf
        }
        _ => fs::read_to_string(from).expect("unable to read from input file"),
    }
}

/// Opens the specified source as input stream.
///
/// Opens the source, or `stdin` if `from` is `"-"`,
/// and returns it as `BufReader`.
/// # Panics
/// An error occurred while opening an input file.
/// This was decided to be able to easily map
/// this function over an iterator.
pub fn open_istream(from: &str) -> BufReader<Box<Read>> {
    let stream: Box<Read> = match from {
        "-" => Box::new(io::stdin()),
        _ => Box::new(File::open(from).expect("unable to open bfin file")),
    };
    BufReader::new(stream)
}

/// Opens the specified source as output stream.
///
/// Opens the source, or `stdout` if `from` is `"-"`,
/// and returns it as `BufWriter`.
/// # Panics
/// An error occurred while opening an output file.
/// This was decided to be able to easily map
/// this function over an iterator.
pub fn open_ostream(from: &str) -> BufWriter<Box<Write>> {
    let stream: Box<Write> = match from {
        "-" => Box::new(io::stdout()),
        _ => Box::new(
            OpenOptions::new()
                .append(true)
                .create(true)
                .open(from)
                .expect("unable to open bfout file"),
        ),
    };
    BufWriter::new(stream)
}
