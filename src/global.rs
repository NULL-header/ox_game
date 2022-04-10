use std::error::Error;
use std::fmt;

#[allow(type_alias_bounds)]
pub type Result<T, U: Error + 'static> = std::result::Result<T, Box<U>>;

#[derive(Debug)]
pub enum CoordinateElement {
  X,
  Y,
}

impl fmt::Display for CoordinateElement {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    let e = match self {
      CoordinateElement::X => "x",
      CoordinateElement::Y => "y",
    };
    write!(f, "{}", e)
  }
}
