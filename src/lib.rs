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

mod context;
mod ops;
mod types;

pub use context::Mode;

use ::unicode_reader::CodePoints;
use context::Context;
use ops::Op;
use std::io::{Read, Write};

/// Interprets a brainfuck program.
///
/// This function takes an input stream of brainfuck instructions and executes
/// them. The optimizations used are determined by the passed mode.
///
/// # Arguments
///
/// * `input` - A reader corresponding to the brainfuck stream.
/// * `bfin` - A reader corresponding to the brainfuck program's stdin.
/// * `bfout` - A reader corresponding to the brainfuck programs stdout.
/// * `mode` - An enum variant that determines the optimization mode.
pub fn run<R: Read, S: Read, W: Write>(input: R, bfin: S, bfout: &mut W, mode: Mode) {
    let mut ctx = Context::new(bfin, bfout, mode);
    let charstream = CodePoints::from(input).map(Result::unwrap);
    let opstream = charstream.filter_map(Op::from);
    ops::exec_all(opstream, &mut ctx)
}

#[cfg(test)]
mod test_bfrun {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn test_hello_world() {
        const HELLO_WORLD: &str = "+[-[<<[+[--->]-[<<<]]]>>>-]>-.---.>..>.<<<<-.<+.>>>>>.>.<<.<-.";
        let hello_buf = HELLO_WORLD.to_owned();
        let bfin = Cursor::new(Vec::new());
        let mut bfout = Vec::new();
        let mode = Mode::Stream;

        run(hello_buf.as_bytes(), bfin, &mut bfout, mode);
        let txtout = String::from_utf8(bfout).expect("unable to decode output as UTF-8");

        println!("Output: {}", txtout);
    }
}
