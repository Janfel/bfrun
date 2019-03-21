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

#[macro_use]
extern crate clap;
extern crate bfrun;

use bfrun::{error::Error, read_file, Interpreter};
use std::{
    env,
    error::Error as StdError,
    fs::{self, File, OpenOptions},
    io::{self, BufReader, BufWriter, Read, BufRead, Write},
};

fn main() -> Result<(), Box<StdError>> {
    let matches = clap_app!(bfrun =>
        (version: crate_version!())
        (author: crate_authors!("\n"))
        (about: crate_description!())
        (@arg istream: -i --bfin [BFIN] "Sets the file the program reads from")
        (@arg ostream: -o --bfout [BFOUT] "Sets the file the program writes to")
        (@arg input: value_name[INPUT] ... "The input file or '-' for stdin")
    )
    .get_matches();

    let mut int = Interpreter::new();

    let mut bfin = matches
        .value_of("istream")
        .map(|x| BufReader::new(File::open(x).expect("unable to open bfin")));

    let mut bfout = matches.value_of("ostream").map(|x| {
        BufWriter::new(
            OpenOptions::new()
                .write(true)
                .create(true)
                .open(x)
                .expect("unable to open bfout"),
        )
    });

    let mut inputs = matches.values_of("input").map(|x| {
        x.map(|y| {
            if y == "-" {
                let mut buf = String::new();
                io::stdin()
                    .read_to_string(&mut buf)
                    .expect("unable to read from stdin");
                buf
            } else {
                fs::read_to_string(y).expect("unable to read from input file")
            }
            .chars()
            .collect::<Vec<char>>()
        })
    });

    if let Some(ref mut v) = bfin {
        int = int.bfin(v);
    }

    if let Some(ref mut v) = bfout {
        int = int.bfout(v);
    }

    match inputs {
        Some(iter) => {
            for prog in iter {
                int.run(prog)?;
            }
        }
        None => {
            int.run({
                let mut buf = String::new();
                io::stdin()
                    .read_to_string(&mut buf)
                    .expect("unable to read from stdin");
                buf.chars().collect::<Vec<char>>()
            })?;
        }
    }

    Ok(())
}

fn read_prog(from: &str) -> String {
    match from {
        "-" => {
            let mut buf = String::new();
            io::stdin()
                .read_to_string(&mut buf)
                .expect("unable to read from stdin");
            buf
        },
        _ => fs::read_to_string(from).expect("unable to read from input file")
    }
}

fn open_istream(from: &str) -> BufReader<Box<Read>> {
    let stream: Box<Read> = match from {
        "-" => Box::new(io::stdin()),
        _ => Box::new(File::open(from).expect("unable to open bfin file")),
    };
    BufReader::new(stream)
}

fn open_ostream(from: &str) -> BufWriter<Box<Write>> {
    let stream: Box<Write> = match from {
        "-" => Box::new(io::stdout()),
        _ => Box::new(OpenOptions::new().append(true).create(true).open(from).expect("unable to open bfout file")),
    };
    BufWriter::new(stream)
}


