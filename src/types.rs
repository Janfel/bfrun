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

use std::collections::HashMap;

/// A struct for buffering chars.
///
/// Useful for counting chars before batch-executing them.
#[derive(Default)]
pub struct CharBuf {
    /// The char that is currently being buffered, if any.
    pub ch_opt: Option<char>,
    /// Counts the number of instances in the buffer.
    pub ctr: u32,
}

impl CharBuf {
    /// Inserts a char into the buffer and increments `ctr`.
    ///
    /// # Panics
    /// If the buffer currently contains a char other than `c`.
    pub fn insert(&mut self, c: char) {
        match self.ch_opt {
            Some(val) if val == c => self.ctr += 1,
            Some(_) => panic!("inserted new char into non-empty CharBuf"),
            None => {
                self.ch_opt = Some(c);
                self.ctr = 1
            }
        }
    }

    /// Resets the buffer to an empty state.
    pub fn clear(&mut self) {
        self.ch_opt = None;
        self.ctr = 0;
    }
}

/// The strip of memory brainfuck uses.
pub type Strip = HashMap<i64, u8>;
