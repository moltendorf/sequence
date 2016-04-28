use super::Root;

use std::rc::Weak;

pub struct Provider {
  root: Weak<Root>
}

impl Provider {
  pub fn new(root: Weak<Root>) -> Provider {
    Provider {
      root: root
    }
  }
}
