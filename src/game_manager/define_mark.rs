use super::game;
use super::read_line;
use game::mark::Marks;
use read_line::ReadLine;
use std::error::Error;

fn define_mark<'a, T, U: Error>(read_line: &'a ReadLine<T, U>) {
  loop {
    let mut answer = String::new();
    println!("Do you want to play as first? (y/*)");
    match read_line(&mut answer) {
      Ok(_) => {}
      Err(_) => {
        continue;
      }
    }
    let answer: Vec<_> = answer.as_str().chars().collect();
    let first = answer.first();
    let first = match first {
      Some(first) => first,
      None => continue,
    };
    if *first != 'y' {
      continue;
    }
  }
}
