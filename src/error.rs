//! Errors that can occurr during brainfuck execution.

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

use std::{error::Error as StdError, fmt, result::Result as StdResult};

#[derive(Copy, Debug, Clone, PartialEq, Eq)]
pub enum Error {
    MismatchedBracket,
    NoInputFile,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use Error::*;
        match self {
            MismatchedBracket => write!(f, "mismatched bracket"),
            NoInputFile => write!(f, "no input file"),
        }
    }
}

impl StdError for Error {}

pub type Result = StdResult<(), Error>;
