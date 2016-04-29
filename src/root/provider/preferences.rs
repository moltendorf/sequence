use super::super::Root;

use std::rc::Weak;

pub struct Preferences {
  root: Weak<Root>
}

impl Preferences {
  pub fn new(root: Weak<Root>) -> Preferences {
    Preferences {
      root: root
    }
  }
}
