use super::game;
use super::input_usize;
use super::read_line;
use crate::Result;
use field::point;
use game::field;
use game::judge::JudgeResult;
use game::Game;
use input_usize::{get_input_usize, InputUsizeError};
use read_line::ReadLine;
use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum PutError<T: Error + 'static> {
  Point(point::TooLargeError),
  Duplicate(field::DuplicatePutError),
  Input(input_usize::InputUsizeError<T>),
}

fn make_error_msg<T: Error + 'static>(e: &PutError<T>) -> String {
  let e = match e {
    PutError::Point(e) => format!("{}", e),
    PutError::Duplicate(e) => format!("{}", e),
    PutError::Input(e) => format!("{}", e),
  };
  format!("Error occurred on the putting.\nError detail: {}", e)
}

impl<T: Error + 'static> fmt::Display for PutError<T> {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{}", make_error_msg(self))
  }
}

impl<T: Error + 'static> Error for PutError<T> {
  fn source(&self) -> Option<&(dyn Error + 'static)> {
    None
  }
}
pub fn put<'a, T, U: Error + 'static>(
  read_line: &'a ReadLine<T, U>,
  game: &mut Game,
) -> Result<Option<JudgeResult>, PutError<U>> {
  println!("Put x:");
  let x = match get_input_usize(read_line) {
    Ok(n) => n,
    Err(e) => {
      return Err(PutError::Input(*e).into());
    }
  };
  println!("Put y:");
  let y = match get_input_usize(read_line) {
    Ok(n) => n,
    Err(e) => {
      return Err(PutError::Input(*e).into());
    }
  };
  let handle = match game.new_point(x, y) {
    Ok(n) => n,
    Err(e) => {
      return Err(PutError::Point(*e).into());
    }
  };
  match handle.put() {
    Ok(n) => Ok(n),
    Err(e) => Err(PutError::Duplicate(*e).into()),
  }
}
