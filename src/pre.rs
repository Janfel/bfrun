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
use crate::error::{Error, Result};
use std::{result::Result as StdResult, sync::mpsc};

trait Analyze {
    fn analyze(&mut self, c: char) -> Result;
}

pub fn process<T>(prog: T) -> StdResult<Vec<char>, Vec<Error>>
where
    T: IntoIterator<Item = char>,
{
    let mut analyzers: Vec<&mut Analyze> = Vec::new();
    let (tx, rx) = mpsc::channel();
    let res = prog
        .into_iter()
        .filter(|x| VALID_CHARS.contains(x))
        .inspect(|x| {
            for a in analyzers.iter_mut() {
                if let Err(e) = a.analyze(*x) {
                    tx.send(e).unwrap()
                }
            }
        })
        .collect();

    let errs: Vec<Error> = rx.into_iter().collect();
    if !errs.is_empty() {
        Err(errs)
    } else {
        Ok(res)
    }
}
