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

use super::VALID_CHARS;
use crate::error::{Error, Result as BfResult};

pub fn process(prog: &[char]) -> Result<Vec<char>, Error> {
    let res: Vec<char> = prog
        .iter()
        .filter(|x| VALID_CHARS.contains(x))
        .cloned()
        .collect();

    brackets(&res)?;

    Ok(res)
}

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
