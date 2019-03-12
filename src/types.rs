use std::ops::{AddAssign, SubAssign};

/// A stack of bytes.
type Stack = Vec<u8>;

/// The Turing strip used by brainfuck.
///
/// The strip includes a pointer to a specific cell that
/// can be moved forwards and backwards.
#[derive(Default)]
pub struct Strip {
    curr: u8,
    left: Stack,
    right: Stack,
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
