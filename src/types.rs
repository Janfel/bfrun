use std::ops::{AddAssign, SubAssign};

/// Intended only for stack operations.
type Stack<T> = Vec<T>;

/// One of the eight brainfuck operators.
type Op = char;

const VALID_OPS: Vec<Op> = vec!['<', '>', '+', '-', '.', ',', '[', ']'];

pub fn char_to_op(b: char) -> Option<Op> {
    if VALID_OPS.contains(&b) {
        Some(b)
    } else {
        None
    }
}

pub fn exec(op: &Op, ctx: &mut Context) {
    unimplemented!(); // TODO Implement
}

/// An executable list of Ops.
type OpList = Vec<Op>;

pub fn exec_all(ops: &OpList, ctx: &mut Context) {
    for op in ops {
        exec(&op, &mut ctx)
    }
}

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
