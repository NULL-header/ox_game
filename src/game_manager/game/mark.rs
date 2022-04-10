#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Marks {
  First,
  Second,
}

fn flip(mark: &Marks) -> Marks {
  match mark {
    Marks::First => Marks::Second,
    Marks::Second => Marks::First,
  }
}

#[derive(Debug)]
pub struct Mark {
  current: Marks,
}

impl Mark {
  pub fn new() -> Mark {
    Mark {
      current: Marks::First,
    }
  }
  pub fn turn(&mut self) {
    self.current = flip(&self.current);
  }
  pub fn current(&self) -> Marks {
    self.current
  }
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn constract_with_first() {
    let mark = Mark::new();
    match mark.current() {
      Marks::First => {}
      n => {
        panic!("{:?}", n);
      }
    };
  }

  #[test]
  fn turn_current() {
    let mut mark = Mark::new();
    mark.turn();
    match mark.current() {
      Marks::Second => {}
      n => {
        panic!("{:?}", n);
      }
    };
  }
}
