use crate::error::{Error, Result};

pub fn all(prog: &[char]) -> Result {
    brackets(prog)
}

fn brackets(prog: &[char]) -> Result {
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
