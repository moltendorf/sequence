mod modules;
mod preferences;

use super::Root;

use self::modules::Modules;
use self::preferences::Preferences;

use std::cell::RefCell;
use std::rc::Weak;

pub struct Provider {
  root: Weak<RefCell<Root>>,
  modules: Modules,
  preferences: Preferences
}

impl Provider {
  pub fn new(root: Weak<RefCell<Root>>) -> Provider {
    Provider {
      modules: Modules::new(root.clone()),
      preferences: Preferences::new(root.clone()),
      root: root
    }
  }

  pub fn modules(&self) -> &Modules {
    &self.modules
  }

  pub fn preferences(&self) -> &Preferences {
    &self.preferences
  }
}
