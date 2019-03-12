use std::ops::{AddAssign, SubAssign};

/// Intended only for stack operations.
type Stack<T> = Vec<T>;

//region Op

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

/// An executable list of Ops.
type OpList = Vec<Op>;

pub fn exec_all(ops: &OpList, ctx: &mut Context) {
    for op in ops {
        op.exec(ctx)
    }
}

//endregion Op

//region Strip

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

//endregion Strip

//region Context

pub struct Context {
    pub strip: Strip,
    pub loops: Stack<OpList>,
}

//endregion Context
