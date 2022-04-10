pub mod point;
pub mod side;
use super::mark::Marks;
use crate::Result;
use point::Point;
use side::Side;
use std::{error, fmt};

#[derive(Debug, Clone)]
pub struct DuplicatePutError(pub Point);

impl DuplicatePutError {
  pub fn new(point: Point) -> DuplicatePutError {
    DuplicatePutError(point)
  }
}

impl fmt::Display for DuplicatePutError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    let val = match self {
      DuplicatePutError(point) => point,
    };
    write!(f, "Putted yet on {}", val)
  }
}

impl error::Error for DuplicatePutError {
  fn source(&self) -> Option<&(dyn error::Error + 'static)> {
    None
  }
}

pub type View = Vec<Vec<Option<Marks>>>;

#[derive(Debug)]
pub struct Putter<'a> {
  field: &'a mut Field,
  point: Point,
}

#[derive(Debug)]
pub struct Field {
  field: View,
  side: Side,
}

impl Field {
  pub fn new(side: Side) -> Field {
    let field = (0..side.len)
      .map(|_| (0..side.len).map(|_| None).collect())
      .collect();
    Field { field, side }
  }
  pub fn new_point(&self, x: usize, y: usize) -> Result<Point, point::TooLargeError> {
    Point::new(&self.side, x, y)
  }
  pub fn put(&mut self, point: Point, next_mark: Marks) -> Result<(), DuplicatePutError> {
    let row = &mut self.field[point.y];
    let mark = &mut row[point.x];
    match mark {
      None => {
        *mark = Some(next_mark);
        Ok(())
      }
      _ => Err(DuplicatePutError::new(point).into()),
    }
  }
  pub fn view(&self) -> &View {
    &self.field
  }
  pub fn print_view(&self) {
    let lines = self.view();
    for line in lines {
      let line_string = line.iter().fold(String::new(), |mut a, e| {
        let mark = match e {
          Some(n) => match n {
            Marks::First => "o",
            Marks::Second => "x",
          },
          None => "-",
        };
        a.push_str(mark);
        return a;
      });
      println!("{}", line_string);
    }
  }
}

impl fmt::Display for Field {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{}", string_view(self))
  }
}

fn string_view(field: &Field) -> String {
  field.view().iter().fold(String::new(), |mut a, e| {
    let line = e.iter().fold(String::new(), |mut a, e| {
      let mark = match e {
        Some(n) => match n {
          Marks::First => "o",
          Marks::Second => "x",
        },
        None => "-",
      };
      a.push_str(mark);
      return a;
    });
    a.push_str(&line);
    a.push_str("\n");
    a
  })
}

#[cfg(test)]
mod test {
  use super::*;
  use rstest::rstest;
  use std::error::Error;

  #[rstest]
  fn side_per_side(#[values(2, 3, 4)] side: usize) -> Result<(), side::TooSmallError> {
    let side = Side::new(side)?;
    let side_len = side.len;
    let field = Field::new(side);
    for line in field.view() {
      assert_eq!(side_len, line.len());
    }
    Ok(())
  }

  #[rstest]
  fn put_bool(
    #[values(2, 3, 4)] num: usize,
    #[values(Marks::First, Marks::Second)] mark: Marks,
  ) -> Result<(), dyn Error> {
    let side = Side::new(num)?;
    let num = num - 1;
    let mut field = Field::new(side);
    let point = field.new_point(num, num)?;
    field.put(point, mark)?;
    let flat = field.view().iter().flatten().collect::<Vec<_>>();
    let filtered = flat
      .into_iter()
      .filter(|e| match e {
        None => false,
        _ => true,
      })
      .collect::<Vec<_>>();
    assert_eq!(&Some(mark), filtered[0]);
    assert_eq!(1, filtered.len());
    Ok(())
  }
  #[rstest]
  fn fail_put_bool(
    #[values(2, 3, 4)] num: usize,
    #[values(Marks::First, Marks::Second)] first: Marks,
    #[values(Marks::First, Marks::Second)] second: Marks,
  ) -> Result<(), dyn Error> {
    let side = Side::new(num)?;
    let num = num - 1;
    let mut field = Field::new(side);
    let point = field.new_point(num, num)?;
    field.put(point, first)?;
    let point = field.new_point(num, num)?;
    match field.put(point, second) {
      Err(e) => {
        let expected = Point::new(&Side::new(num + 1)?, num, num)?;
        match *e {
          DuplicatePutError(point) => {
            assert_eq!(expected, point);
          }
        }
      }
      Ok(n) => {
        panic!("{:?}", n);
      }
    }
    Ok(())
  }
}
