use crate::Result;
use std::error::Error;
use std::io;

pub fn read_line(string: &mut String) -> Result<usize, io::Error> {
  match io::stdin().read_line(string) {
    Err(e) => Err(Box::new(e)),
    Ok(n) => Ok(n),
  }
}

#[allow(type_alias_bounds)]
pub type ReadLine<T, U: Error> = dyn Fn(&mut String) -> Result<T, U>;
