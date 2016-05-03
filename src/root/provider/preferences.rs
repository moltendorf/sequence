use super::super::Root;

use std::cell::RefCell;
use std::rc::Weak;

pub struct Preferences {
  root: Weak<RefCell<Root>>
}

impl Preferences {
  pub fn new(root: Weak<RefCell<Root>>) -> Preferences {
    Preferences {
      root: root
    }
  }
}
