use super::input_usize;
use super::read_line;
use crate::Result;
use error::define_mode::ModeError;
use input_usize::get_input_usize;
use input_usize::InputUsizeError;
use read_line::ReadLine;
use std::error::Error;

#[derive(Debug)]
pub enum Modes {
  WithBot,
  Battle,
}

mod error {
  pub mod define_mode {
    use super::super::InputUsizeError;
    use std::error::Error;
    use std::fmt;

    #[derive(Debug)]
    pub enum ModeError<T: Error + 'static> {
      Input(InputUsizeError<T>),
      Invalid,
    }
    impl<T: Error> fmt::Display for ModeError<T> {
      fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let msg = match self {
          ModeError::Input(n) => format!("{}", n),
          ModeError::Invalid => {
            format!("The number is invalid. You must put 1 or 2.")
          }
        };
        write!(f, "{}", msg)
      }
    }

    impl<T: Error> Error for ModeError<T> {
      fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
          ModeError::Input(n) => Some(&*n),
          ModeError::Invalid => None,
        }
      }
    }
  }
}

pub fn define_mode<'a, T, U: Error + 'static>(
  read_line: &'a ReadLine<T, U>,
) -> Result<Modes, ModeError<U>> {
  println!(
    "How many do you want to play?\nIf you want to play with a bot, you should put 1. (1/2)"
  );
  let num_player = match get_input_usize(read_line) {
    Ok(n) => n,
    Err(n) => return Err(ModeError::Input(*n).into()),
  };
  let mode = match num_player {
    1 => Modes::WithBot,
    2 => Modes::Battle,
    _ => return Err(ModeError::Invalid.into()),
  };
  Ok(mode)
}

#[cfg(test)]
mod test {
  use super::*;
  use rstest::*;
  use std::error::Error;
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
  #[case(Modes::WithBot, read_line("1"))]
  #[case(Modes::Battle, read_line("2"))]
  fn success_define_mode(#[case] expected: Modes, #[case] read_line: Box<ReadLine>) -> Result {
    let mode = define_mode(&read_line)?;
    match expected {
      Modes::WithBot => match mode {
        Modes::WithBot => {}
        n => {
          panic!("{:?}", n);
        }
      },
      Modes::Battle => match mode {
        Modes::Battle => {}
        n => {
          panic!("{:?}", n);
        }
      },
    };
    Ok(())
  }

  #[rstest]
  #[case(read_line("0"))]
  #[case(read_line("3"))]
  fn fail_define_mode_with_invalid(#[case] read_line: Box<ReadLine>) {
    let mode = define_mode(&read_line);
    match mode {
      Ok(n) => {
        panic!("{:?}", n);
      }
      Err(e) => match *e {
        ModeError::Invalid => {}
        e => {
          panic!("{}", e);
        }
      },
    }
  }
}
