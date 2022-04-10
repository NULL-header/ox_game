use super::read_line::ReadLine;
use crate::Result;
use std::error::Error;
use std::fmt;
use std::num::ParseIntError;

#[derive(Debug)]
pub enum InputUsizeError<T: Error + 'static> {
  ReadLine(T),
  Parse(ParseIntError),
}

impl<T: Error + 'static> fmt::Display for InputUsizeError<T> {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    let msg = match self {
      InputUsizeError::ReadLine(e) => format!("{}", e),
      InputUsizeError::Parse(e) => format!("{}", e),
    };
    write!(f, "{}", msg)
  }
}

impl<T: Error + 'static> Error for InputUsizeError<T> {
  fn source(&self) -> Option<&(dyn Error + 'static)> {
    match self {
      InputUsizeError::ReadLine(e) => Some(e),
      InputUsizeError::Parse(e) => Some(e),
    }
  }
}

pub fn get_input_usize<T, U: Error + 'static>(
  read_line: &ReadLine<T, U>,
) -> Result<usize, InputUsizeError<U>> {
  let mut s = String::new();
  match read_line(&mut s) {
    Ok(_) => {}
    Err(e) => return Err(InputUsizeError::ReadLine(*e).into()),
  };
  let num = match s.trim().parse::<usize>() {
    Ok(n) => n,
    Err(e) => return Err(InputUsizeError::Parse(e).into()),
  };
  Ok(num)
}

#[cfg(test)]
mod test {
  use super::*;
  use rstest::*;
  use std::io;
  type Result = super::Result<(), dyn Error>;
  type ReadLine = super::ReadLine<(), io::Error>;

  #[fixture]
  fn read_line(#[default("1")] input: &'static str) -> Box<ReadLine> {
    let read_line = |s: &mut String| {
      s.push_str(input);
      Ok(())
    };
    Box::new(read_line)
  }

  #[rstest]
  fn success(read_line: Box<ReadLine>) -> Result {
    let result = get_input_usize(&read_line)?;
    assert_eq!(1, result);
    Ok(())
  }

  #[rstest]
  fn parse_minus(#[with("-1")] read_line: Box<ReadLine>) {
    let result = get_input_usize(&read_line);
    match result {
      Err(e) => match *e {
        InputUsizeError::Parse(_) => {}
        n => {
          panic!("{}", n);
        }
      },
      n => {
        panic!("{:?}", n);
      }
    }
  }

  #[rstest]
  fn parse_chars(#[with("abc")] read_line: Box<ReadLine>) {
    let result = get_input_usize(&read_line);
    match result {
      Err(e) => match *e {
        InputUsizeError::Parse(_) => {}
        n => {
          panic!("{}", n);
        }
      },
      n => {
        panic!("{:?}", n);
      }
    }
  }
}
