use super::Root;

use std::rc::Weak;

pub struct Application {
  root: Weak<Root>
}

impl Application {
  pub fn new(root: Weak<Root>) -> Application {
    Application {
      root: root
    }
  }
}
