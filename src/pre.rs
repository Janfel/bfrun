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

//! Functions for preprocessing the program string.
//!
//! The main function of this module is `process()`,
//! which converts a string into a statically analyzed
//! vector of brainfuck operators.

use crate::error::{Error, Result as BfResult};

/// All valid brainfuck operators.
const VALID_CHARS: [char; 8] = ['+', '-', '<', '>', '.', ',', '[', ']'];

/// Turns the given program into a vector of valid chars.
///
/// Converts a string into a statically analyzed
/// vector of brainfuck operators. Every insignificant
/// char is filtered out.
/// # Result
/// The static analysis found a logic error in the program
pub fn process(prog: &str) -> Result<Vec<char>, Error> {
    let res: Vec<char> = prog.chars().filter(|x| VALID_CHARS.contains(x)).collect();

    brackets(&res)?;

    Ok(res)
}

/// Checks the program for mismatched brackets.
fn brackets(prog: &[char]) -> BfResult {
    let mut acc = 0;
    for c in prog.iter() {
        match c {
            '[' => acc += 1,
            ']' => {
                if acc == 0 {
                    return Err(Error::MissingLeftBracket);
                } else {
                    acc -= 1
                }
            }
            _ => (),
        }
    }
    if acc != 0 {
        return Err(Error::MissingRightBracket);
    }
    Ok(())
}
