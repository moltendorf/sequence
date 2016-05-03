mod preferences;

use super::Root;

use self::preferences::Preferences;

use std::cell::RefCell;
use std::rc::Weak;

pub struct Provider {
  root: Weak<RefCell<Root>>,
  preferences: Preferences
}

impl Provider {
  pub fn new(root: Weak<RefCell<Root>>) -> Provider {
    Provider {
      preferences: Preferences::new(root.clone()),
      root: root
    }
  }

  pub fn preferences(&self) -> &Preferences {
    &self.preferences
  }
}
