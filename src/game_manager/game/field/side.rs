use crate::Result;

use std::error;
use std::fmt;

#[derive(Debug, Clone)]
pub struct TooSmallError;

impl fmt::Display for TooSmallError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "the side is too small to make field.")
  }
}

impl error::Error for TooSmallError {
  fn source(&self) -> Option<&(dyn error::Error + 'static)> {
    None
  }
}

#[derive(Debug)]
pub struct Side {
  pub len: usize,
}

impl Side {
  pub fn new(len: usize) -> Result<Side, TooSmallError> {
    if len < 2 {
      return Err(TooSmallError.into());
    }
    Ok(Side { len })
  }
}

#[cfg(test)]
mod test {
  use super::*;
  use rstest::rstest;

  #[test]
  fn too_small_side() {
    match Side::new(1) {
      Err(_) => {}
      Ok(n) => {
        panic!("{:?}", n);
      }
    };
  }

  #[rstest]
  fn new_side(#[values(2, 3, 4)] len: usize) -> Result<(), TooSmallError> {
    let side = Side::new(len)?;
    assert_eq!(side.len, len);
    Ok(())
  }
}
