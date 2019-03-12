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

use std::ops::{AddAssign, SubAssign};

/// A vector intended only for use with stack operations.
pub type Stack<T> = Vec<T>;

/// The Turing strip used by brainfuck.
///
/// The strip includes an address pointer to a specific
/// cell that can be moved forwards and backwards.
#[derive(Default)]
pub struct Strip {
    curr: u8,
    left: Stack<u8>,
    right: Stack<u8>,
}

impl Strip {
    /// Creates a new Strip.
    pub fn new() -> Self {
        Self::default()
    }
    /// The value of the cell the pointer currently points at.
    pub fn get(&self) -> u8 {
        self.curr
    }
    /// Moves the pointer.
    ///
    /// This function moves the pointer any amount of steps
    /// in the positive or negative direction. It is
    /// recommended to buffer the input beforehand.
    pub fn mov(&mut self, mut steps: isize) {
        if steps == 0 {
            return;
        }

        let (from, to) = if steps < 0 {
            (&mut self.left, &mut self.right)
        } else {
            (&mut self.right, &mut self.left)
        };

        let usteps = if steps == isize::min_value() {
            (steps + 1).abs() as usize
        } else {
            steps.abs() as usize - 1
        };

        to.push(self.curr);
        for _ in 0..usteps {
            to.push(from.pop().unwrap_or_default());
        }
        self.curr = from.pop().unwrap_or_default();
    }
}
impl AddAssign<u8> for Strip {
    /// Increments the value of the current cell.
    ///
    /// This function wraps around on byte overflow as
    /// specified by the unofficial brainfuck specification.
    fn add_assign(&mut self, other: u8) {
        self.curr = self.curr.wrapping_add(other);
    }
}
impl SubAssign<u8> for Strip {
    /// Decrements the value of the current cell.
    ///
    /// This function wraps around on byte underflow as
    /// specified by the unofficial brainfuck specification.
    fn sub_assign(&mut self, other: u8) {
        self.curr = self.curr.wrapping_sub(other);
    }
}
