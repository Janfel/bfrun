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

use std::io::{Read, Write};

/// Interprets a brainfuck file.
///
/// This function takes an input stream and treats it
/// as a brainfuck file. Does more optimizations than
/// `run_stream`, but blocks when encountering a loop.
///
/// # Arguments
///
/// * `input` - A reader containing the brainfuck file.
/// * `bfin` - A reader corresponding to the brainfuck program's stdin.
/// * `bfout` - A reader corresponding to the brainfuck programs stdout.
pub fn run_file<R, W>(input: R, bfin: R, bfout: W)
where
    R: Read,
    W: Write,
{
    unimplemented!();
}
/// Interprets a brainfuck stream.
///
/// This function takes an input stream of brainfuck instructions and executes
/// them. Does less optimizations than `run_file`, but does not block when
/// encountering a loop.
///
/// # Arguments
///
/// * `input` - A reader corresponding to the brainfuck stream.
/// * `bfin` - A reader corresponding to the brainfuck program's stdin.
/// * `bfout` - A reader corresponding to the brainfuck programs stdout.
pub fn run_stream<R, W>(input: R, bfin: R, bfout: W)
where
    R: Read,
    W: Write,
{
    unimplemented!();
}
