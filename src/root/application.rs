use super::Root;

use std::cell::RefCell;
use std::rc::Weak;

pub struct Application {
  root: Weak<RefCell<Root>>
}

impl Application {
  pub fn new(root: Weak<RefCell<Root>>) -> Application {
    Application {
      root: root
    }
  }
}
