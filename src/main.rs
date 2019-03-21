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

use bfrun::Interpreter;
use std::{
    error::Error as StdError,
    fs::{self, File, OpenOptions},
    io::{self, BufReader, BufWriter, Read, Write},
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

    let mut inter = Interpreter::new();

    let mut intemp = matches.value_of("istream").map(open_istream);
    if let Some(ref mut rd) = intemp {
        inter = inter.bfin(rd);
    }

    let mut outtemp = matches.value_of("ostream").map(open_ostream);
    if let Some(ref mut wt) = outtemp {
        inter = inter.bfout(wt);
    }

    match matches.values_of("input") {
        Some(it) => {
            for prog in it.map(read_prog) {
                inter.run(prog.chars().collect())?;
            }
        }
        None => {
            inter.run(read_prog("-").chars().collect())?;
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
        }
        _ => fs::read_to_string(from).expect("unable to read from input file"),
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
