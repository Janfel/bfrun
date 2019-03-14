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
use std::iter;

/// One of the eight brainfuck operators.
#[derive(Copy, Clone, Eq, PartialEq)]
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
    pub fn from(c: char) -> Option<Self> {
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

    pub fn exec(self, mut num: u32, mut ctx: &mut Context) {
        use Op::*;
        for op_loop in ctx.loops.iter_mut() {
            op_loop.push(self)
        }
        if ctx.skip_ctr != 0 {
            match self {
                LoopL => ctx.skip_ctr += num,
                LoopR => {
                    if num >= ctx.skip_ctr {
                        num -= ctx.skip_ctr;
                        ctx.skip_ctr = 0;
                    } else {
                        ctx.skip_ctr -= num
                    }
                }
                _ => {}
            };
            if ctx.skip_ctr != 0 {
                return;
            };
        }
        match self {
            Left => ctx.strip.move_left(num),
            Right => ctx.strip.move_right(num),
            Inc => ctx.strip += as_u8(num),
            Dec => ctx.strip -= as_u8(num),
            Put => {
                ctx.bfout
                    .write_all(&vec![ctx.strip.get(); num as usize])
                    .expect("failed to write buffer to bfout");
            }
            Get => {
                let mut buf = vec![0; num as usize];
                ctx.bfin
                    .read_exact(&mut buf)
                    .expect("failed to read from bfin");
                ctx.strip.set(*buf.last().unwrap())
            }
            LoopL => {
                if ctx.strip.get() == 0 {
                    ctx.skip_ctr = num
                } else {
                    for _ in 0..num {
                        ctx.loops.push(Vec::new())
                    }
                }
            }
            LoopR => {
                for _ in 0..num {
                    let op_loop = ctx.loops.pop().expect("Mismatched bracket.");
                    while ctx.strip.get() != 0 {
                        exec_all(op_loop.clone(), &mut ctx)
                    }
                }
            }
        };
    }
}

pub fn exec_all<I>(ops: I, mut ctx: &mut Context)
where
    I: IntoIterator<Item = Op>,
{
    let mut opstream = ops.into_iter();
    let startop = match opstream.next() {
        Some(v) => v,
        None => return,
    };
    let facc = opstream.fold((startop, 1), |acc, x| {
        if x == acc.0 {
            (x, acc.1 + 1)
        } else {
            acc.0.exec(acc.1, &mut ctx);
            (x, 1)
        }
    });
    facc.0.exec(facc.1, &mut ctx);
}

fn as_u8(mut num: u32) -> u8 {
    let umax = std::u8::MAX.into();
    while num > umax {
        num -= umax
    }
    num as u8
}
