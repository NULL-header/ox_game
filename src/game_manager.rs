mod define_mark;
mod define_mode;
mod game;
mod initialize;
mod input_usize;
mod put_by;
mod read_line;
use crate::Result;
use define_mode::{define_mode, Modes};
use game::judge::JudgeResult;
use game::mark::Marks;
use game::Game;
use initialize::initialize;
use read_line::{read_line, ReadLine};
use std::error::Error;
use std::io;

#[derive(Debug)]
pub struct GameManager {
  game: Game,
  mode: Modes,
}

fn loop_f<'a, T, U, V>(target: &'a mut T) -> U
where
  V: Error + 'static,
  T: for<'b> FnMut(&'b ReadLine<usize, io::Error>) -> Result<U, V>,
{
  match target(&read_line) {
    Ok(n) => n,
    Err(n) => {
      println!("{}", n);
      println!("Please retry.");
      loop_f(target)
    }
  }
}

impl GameManager {
  pub fn new() -> GameManager {
    println!("put the side of the field that you want to play it.");
    let game = loop_f(&mut initialize);
    let mode = loop_f(&mut define_mode);
    GameManager { game, mode }
  }
  fn put_by_bot(&mut self) {
    // random
  }
  fn play_base<'a, T, U>(&mut self, second: &'a mut T) -> JudgeResult
  where
    U: Error + 'static,
    T: FnMut(&ReadLine<usize, io::Error>, &mut Game) -> Result<Option<JudgeResult>, U>,
  {
    let loop_put_first =
      |game: &mut Game| loop_f(&mut |read_line| put_by::player::put(read_line, game));
    let mut loop_put_second = |game: &mut Game| loop_f(&mut |read_line| second(read_line, game));
    loop {
      println!("{}", self.game);
      if let Some(n) = loop_put_first(&mut self.game) {
        break n;
      }
      println!("{}", self.game);
      if let Some(n) = loop_put_second(&mut self.game) {
        break n;
      }
    }
  }
  fn battle(&mut self) -> JudgeResult {
    self.play_base(&mut put_by::player::put)
    // play base call with put by player
  }
  fn with_bot(&mut self) {
    // define first or second
    // if second, call put by bot
    // play_base call with put by bot
  }
  pub fn play(&mut self) {
    println!("start game!");
    let result = match self.mode {
      Modes::Battle => self.battle(),
      Modes::WithBot => self.battle(),
    };
    println!("{}", self.game);
    println!(
      "winner is {}",
      match result {
        JudgeResult::Win(n) => match n {
          Marks::First => "first",
          Marks::Second => "second",
        },
        JudgeResult::Draw => "None. it is draw.",
      }
    )
  }
}
