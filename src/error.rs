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

//! Errors that occur during compile-time or runtime.
//!
//! This module contains the `Error` and `Result` types
//! used by the bfrun interpreter.

use std::{error::Error as StdError, fmt, result::Result as StdResult};

/// Errors that occur while parsing and checking the program.
#[derive(Copy, Debug, Clone, PartialEq, Eq)]
pub enum Error {
    /// There are more left brackets than right brackets in the program.
    MissingRightBracket,
    /// There are more right brackets than left brackets in the program.
    MissingLeftBracket,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use Error::*;
        match self {
            MissingRightBracket => write!(f, "Error: MissingRightBracket"),
            MissingLeftBracket => write!(f, "Error: MissingLeftBracket"),
        }
    }
}

impl StdError for Error {}

/// A value indicating if an error occurred during execution.
pub type Result = StdResult<(), Error>;
