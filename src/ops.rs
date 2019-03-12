//! Types for encoding and executing single operations.

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
