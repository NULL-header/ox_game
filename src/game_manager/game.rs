pub mod field;
pub mod judge;
pub mod mark;
use crate::Result;
use field::point;
use field::side::Side;
use field::Field;
use judge::judge_winner;
use judge::JudgeResult;
use mark::Mark;
use point::Point;
use std::fmt;

fn turn<'a>(field: &'a Field, mark: &'a mut Mark) -> Option<JudgeResult> {
  let winner = judge_winner(field.view());
  mark.turn();
  winner
}

pub struct Putter<'a> {
  mark: &'a mut Mark,
  point: Point,
  field: &'a mut Field,
}

impl<'a> Putter<'a> {
  pub fn put(self) -> Result<Option<JudgeResult>, field::DuplicatePutError> {
    self.field.put(self.point, self.mark.current())?;
    Ok(turn(self.field, self.mark))
  }
}

#[derive(Debug)]
pub struct Game {
  mark: Mark,
  field: Field,
}

impl Game {
  pub fn new(side: Side) -> Game {
    let mark = Mark::new();
    let field = Field::new(side);
    Game { mark, field }
  }
  pub fn new_point(&mut self, x: usize, y: usize) -> Result<Putter, point::TooLargeError> {
    let point = self.field.new_point(x, y)?;
    Ok(Putter {
      field: &mut self.field,
      mark: &mut self.mark,
      point,
    })
  }
  pub fn print_field(&self) {
    self.field.print_view();
  }
}

impl fmt::Display for Game {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{}", self.field)
  }
}

#[cfg(test)]
mod test {
  use super::mark::Marks;
  use super::*;
  use rstest::*;
  type Result = super::Result<(), dyn std::error::Error>;

  #[fixture]
  pub fn game(#[default(2)] len: usize) -> Game {
    let side = Side::new(len).unwrap();
    Game::new(side)
  }

  #[rstest]
  fn turn_mark_with_put(mut game: Game) -> Result {
    match game.mark.current() {
      Marks::First => {}
      n => {
        panic!("{:?}", n);
      }
    }
    game.new_point(0, 0)?.put()?;

    match game.mark.current() {
      Marks::Second => {}
      n => {
        panic!("{:?}", n);
      }
    }
    Ok(())
  }
  #[rstest]
  fn winner_first_with_put(mut game: Game) -> Result {
    game.new_point(0, 0)?.put()?;
    game.new_point(0, 1)?.put()?;
    let result = game.new_point(1, 1)?.put()?;
    match result {
      Some(JudgeResult::Win(Marks::First)) => {}
      n => {
        panic!("{:?}", n);
      }
    };
    Ok(())
  }
  #[rstest]
  fn winner_second_with_put(#[with(3)] mut game: Game) -> Result {
    game.new_point(0, 1)?.put()?;
    game.new_point(0, 0)?.put()?;
    game.new_point(0, 2)?.put()?;
    game.new_point(1, 1)?.put()?;
    game.new_point(1, 0)?.put()?;
    let result = game.new_point(2, 2)?.put()?;
    match result {
      Some(JudgeResult::Win(Marks::Second)) => {}
      n => {
        panic!("{:?}", n);
      }
    };
    Ok(())
  }
  #[rstest]
  fn winner_none_with_put(mut game: Game) -> Result {
    let result = game.new_point(0, 0)?.put()?;
    match result {
      None => {}
      n => {
        panic!("{:?}", n);
      }
    };
    Ok(())
  }
}
