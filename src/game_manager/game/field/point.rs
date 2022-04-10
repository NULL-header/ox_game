pub use super::side::Side;
use crate::{CoordinateElement, Result};
use std::error;
use std::fmt;

#[derive(Debug)]
pub struct TooLargeError(CoordinateElement);

fn make_error_msg(e: &TooLargeError) -> String {
  let target = match e {
    TooLargeError(e) => e,
  };
  format!("{} is too large.", target)
}

impl fmt::Display for TooLargeError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{}", make_error_msg(self))
  }
}

impl error::Error for TooLargeError {
  fn source(&self) -> Option<&(dyn error::Error + 'static)> {
    None
  }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Point {
  pub x: usize,
  pub y: usize,
}

impl Point {
  pub fn new(side: &Side, x: usize, y: usize) -> Result<Point, TooLargeError> {
    if x >= side.len {
      return Err(TooLargeError(CoordinateElement::X).into());
    }
    if y >= side.len {
      return Err(TooLargeError(CoordinateElement::Y).into());
    }
    Ok(Point { x, y })
  }
}

impl fmt::Display for Point {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "(x:{}, y:{}).", self.x, self.y)
  }
}

#[cfg(test)]
mod test {
  use super::*;
  use rstest::rstest;
  type Result = super::Result<(), dyn std::error::Error>;

  #[rstest]
  fn too_large_x(#[values(2, 3, 4)] side_len: usize) -> Result {
    let side = Side::new(side_len)?;
    match Point::new(&side, side_len, 0) {
      Err(e) => match *e {
        TooLargeError(CoordinateElement::X) => {}
        n => {
          panic!("{}", n);
        }
      },
      n => {
        panic!("{:?}", n);
      }
    };
    Ok(())
  }
  #[rstest]
  fn too_large_y(#[values(2, 3, 4)] side_len: usize) -> Result {
    let side = Side::new(side_len)?;
    match Point::new(&side, 0, side_len) {
      Err(e) => match *e {
        TooLargeError(CoordinateElement::Y) => {}
        n => {
          panic!("{}", n);
        }
      },
      n => {
        panic!("{:?}", n);
      }
    };
    Ok(())
  }
}
