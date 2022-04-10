mod make_lines;
use super::field::View;
use super::mark::Marks;
use make_lines::{make_lines, Lines};

#[derive(Debug)]
pub enum JudgeResult {
  Win(Marks),
  Draw,
}

fn judge_winner_core(lines: Lines) -> Option<JudgeResult> {
  let mut winner: Option<Marks> = None;
  let mut has_none = false;
  for line in lines {
    let first = line[0];
    if *first == None {
      has_none = true;
    }
    let first = match first {
      None => continue,
      Some(n) => n,
    };
    let does_finish = line[1..].into_iter().all(|&e| match e {
      None => {
        if !has_none {
          has_none = true;
        }
        false
      }
      Some(e) => e == first,
    });
    if does_finish {
      winner = Some(*first);
      break;
    }
  }
  match winner {
    None => {
      if has_none {
        None
      } else {
        Some(JudgeResult::Draw)
      }
    }
    Some(n) => Some(JudgeResult::Win(n)),
  }
}

pub fn judge_winner(view: &View) -> Option<JudgeResult> {
  let lines = make_lines(view);
  judge_winner_core(lines)
}

#[cfg(test)]
mod test {
  use super::*;
  use rstest::rstest;

  #[rstest]
  fn core_win(#[values(Marks::First, Marks::Second)] mark: Marks) {
    let putted = &Some(mark);
    let val = vec![vec![putted, putted]];
    let result = judge_winner_core(val).unwrap();
    match result {
      JudgeResult::Win(n) => {
        assert_eq!(n, mark);
      }
      n => {
        panic!("{:?}", n);
      }
    };
  }

  #[rstest]
  fn core_draw(#[values(Marks::First, Marks::Second)] mark: Marks) {
    let left = &Some(mark);
    let right = &Some(match mark {
      Marks::First => Marks::Second,
      Marks::Second => Marks::First,
    });
    let val = vec![vec![left, right]];
    let result = judge_winner_core(val).unwrap();
    match result {
      JudgeResult::Draw => {}
      n => {
        panic!("{:?}", n);
      }
    };
  }

  #[rstest]
  fn core_not_yet(#[values(Some(Marks::First), Some(Marks::Second), None)] right: Option<Marks>) {
    let val = vec![vec![&None, &right]];
    let result = judge_winner_core(val);
    match result {
      None => {}
      n => {
        panic!("{:?}", n);
      }
    };
  }
}
