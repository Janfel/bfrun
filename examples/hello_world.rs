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

use bfrun::{read_file, Interpreter};
use std::{error::Error, io};

const FNAME: &str = "examples/hello_world.b";

fn main() -> Result<(), Box<Error>> {
    let prog = read_file(FNAME)?;
    let mut bfin = io::stdin();
    let mut bfout = io::stdout();
    Interpreter::new(&mut bfin, &mut bfout).run(&prog)?;
    Ok(())
}
