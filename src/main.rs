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

extern crate bfrun;

use bfrun::{error::Error, Interpreter};
use std::{env, error::Error as StdError, fs};

fn main() -> Result<(), Box<StdError>> {
    let fname = env::args().nth(1).ok_or(Error::NoInputFile)?;
    let prog: Vec<char> = fs::read_to_string(fname)?.chars().collect();

    Interpreter::new().run(prog)?;
    Ok(())
}
