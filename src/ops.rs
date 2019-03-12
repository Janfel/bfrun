//! Types for encoding and executing single operations.

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

use crate::context::Context;

/// One of the eight brainfuck operators.
#[derive(Copy, Clone)]
pub enum Op {
    Left,
    Right,
    Inc,
    Dec,
    Put,
    Get,
    LoopL,
    LoopR,
}

/// An executable list of Ops.
pub type OpList = Vec<Op>;

impl Op {
    pub fn from_char(c: char) -> Option<Self> {
        use Op::{Dec, Get, Inc, Left, LoopL, LoopR, Put, Right};
        match c {
            '<' => Some(Left),
            '>' => Some(Right),
            '+' => Some(Inc),
            '-' => Some(Dec),
            '.' => Some(Put),
            ',' => Some(Get),
            '[' => Some(LoopL),
            ']' => Some(LoopR),
            _ => None,
        }
    }

    pub fn to_char(&self) -> char {
        use Op::{Dec, Get, Inc, Left, LoopL, LoopR, Put, Right};
        match self {
            Left => '<',
            Right => '>',
            Inc => '+',
            Dec => '-',
            Put => '.',
            Get => ',',
            LoopL => '[',
            LoopR => ']',
        }
    }

    pub fn exec(&self, ctx: &mut Context) {
        unimplemented!(); // TODO Implement
    }
}

pub fn exec_all(ops: &OpList, ctx: &mut Context) {
    for op in ops {
        op.exec(ctx)
    }
}
