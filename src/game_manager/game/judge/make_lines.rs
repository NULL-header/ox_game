use super::Marks;
use super::View;

pub type Lines<'a> = Vec<Vec<&'a Option<Marks>>>;

fn make_lines_horizontal(view: &View) -> Lines {
  view.iter().map(|e| e.iter().collect()).collect()
}

fn make_lines_vertical(view: &View) -> Lines {
  let side_len = view.len();
  (0..side_len)
    .into_iter()
    .map(|x| (0..side_len).into_iter().map(|y| &view[y][x]).collect())
    .collect()
}

fn make_lines_slash(view: &View) -> Lines {
  let side_len = view.len();
  let final_index = side_len - 1;
  vec![(0..side_len)
    .into_iter()
    .map(|x| &view[x][final_index - x])
    .collect()]
}

fn make_lines_backslash(view: &View) -> Lines {
  vec![(0..view.len()).into_iter().map(|x| &view[x][x]).collect()]
}

pub fn make_lines(view: &View) -> Lines {
  let results = vec![
    make_lines_horizontal(view),
    make_lines_vertical(view),
    make_lines_slash(view),
    make_lines_backslash(view),
  ];
  let all = results.into_iter().reduce(|mut a, e| {
    a.extend(e);
    a
  });
  match all {
    None => panic!("reduce failed."),
    Some(n) => n,
  }
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn horizontal() {
    let val = vec![
      vec![Some(Marks::First), Some(Marks::First)],
      vec![Some(Marks::Second), Some(Marks::Second)],
    ];
    let result = make_lines_horizontal(&val);
    assert_eq!(
      vec![
        vec![&Some(Marks::First), &Some(Marks::First)],
        vec![&Some(Marks::Second), &Some(Marks::Second)],
      ],
      result
    );
  }

  #[test]
  fn vertical() {
    let val = vec![
      vec![Some(Marks::First), Some(Marks::First)],
      vec![Some(Marks::Second), Some(Marks::Second)],
    ];
    let result = make_lines_vertical(&val);
    assert_eq!(
      vec![
        vec![&Some(Marks::First), &Some(Marks::Second)],
        vec![&Some(Marks::First), &Some(Marks::Second)],
      ],
      result
    )
  }

  #[test]
  fn slash() {
    let val = vec![
      vec![Some(Marks::First), Some(Marks::Second)],
      vec![Some(Marks::Second), Some(Marks::First)],
    ];
    let result = make_lines_slash(&val);
    assert_eq!(
      vec![vec![&Some(Marks::Second), &Some(Marks::Second)]],
      result
    );
  }

  #[test]
  fn backslash() {
    let val = vec![
      vec![Some(Marks::First), Some(Marks::Second)],
      vec![Some(Marks::Second), Some(Marks::First)],
    ];
    let result = make_lines_backslash(&val);
    assert_eq!(vec![vec![&Some(Marks::First), &Some(Marks::First)]], result);
  }

  #[test]
  fn all() {
    let val = vec![
      vec![Some(Marks::First), Some(Marks::First)],
      vec![Some(Marks::Second), Some(Marks::Second)],
    ];
    let result = make_lines(&val);
    assert_eq!(
      vec![
        vec![&Some(Marks::First), &Some(Marks::First)],
        vec![&Some(Marks::Second), &Some(Marks::Second)],
        vec![&Some(Marks::First), &Some(Marks::Second)],
        vec![&Some(Marks::First), &Some(Marks::Second)],
        vec![&Some(Marks::First), &Some(Marks::Second)],
        vec![&Some(Marks::First), &Some(Marks::Second)],
      ],
      result
    );
  }
}
