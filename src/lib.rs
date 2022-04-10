mod game_manager;
mod global;

use game_manager::GameManager;
pub use global::CoordinateElement;
pub use global::Result;

pub fn run() {
  let mut manager = GameManager::new();
  manager.play();
  println!("{:?}", manager);
}
