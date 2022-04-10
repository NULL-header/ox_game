use super::game;
use super::input_usize;
use super::read_line;
use crate::Result;
use field::side::Side;
use game::{field, Game};
use input_usize::get_input_usize;
use input_usize::InputUsizeError;
use read_line::ReadLine;
use std::error::Error;

use game::field::side::TooSmallError;
use std::fmt;

#[derive(Debug)]
pub enum InitializeError<T: Error + 'static> {
  Input(InputUsizeError<T>),
  Start(TooSmallError),
}

impl<T: Error> fmt::Display for InitializeError<T> {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    let msg = match self {
      InitializeError::Input(n) => format!("{}", n),
      InitializeError::Start(n) => {
        format!("An error occurred to start game.\nError detail: {}", n)
      }
    };
    write!(f, "{}", msg)
  }
}

impl<T: Error> Error for InitializeError<T> {
  fn source(&self) -> Option<&(dyn Error + 'static)> {
    match self {
      InitializeError::Input(n) => Some(n),
      InitializeError::Start(n) => Some(n),
    }
  }
}

pub fn initialize<'a, T, U: Error>(
  read_line: &'a ReadLine<T, U>,
) -> Result<Game, InitializeError<U>> {
  let game: Game;
  let side = match get_input_usize(read_line) {
    Ok(n) => n,
    Err(n) => {
      return Err(InitializeError::Input(*n).into());
    }
  };
  match Side::new(side) {
    Err(n) => {
      return Err(InitializeError::Start(*n).into());
    }
    Ok(side) => {
      game = Game::new(side);
    }
  }
  Ok(game)
}

#[cfg(test)]
mod test {
  use super::*;
  use rstest::*;
  use std::io;

  type Result = super::Result<(), dyn Error>;
  type ReadLine = super::ReadLine<(), io::Error>;

  #[fixture]
  fn read_line(#[default("3")] input: &'static str) -> Box<ReadLine> {
    let read_line = |s: &mut String| {
      s.push_str(input);
      Ok(())
    };
    Box::new(read_line)
  }

  #[rstest]
  fn success_initialize(read_line: Box<ReadLine>) -> Result {
    initialize(&read_line)?;
    Ok(())
  }
  #[rstest]
  fn fail_initialize_with_too_small(#[with("1")] read_line: Box<ReadLine>) {
    match initialize(&read_line) {
      Ok(n) => {
        panic!("{:?}", n);
      }
      Err(e) => match *e {
        InitializeError::Start(_) => {}
        e => {
          panic!("{}", e);
        }
      },
    };
  }
}
