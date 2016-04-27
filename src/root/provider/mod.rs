use super::Root;

pub struct Provider {
  root: &Root
}

impl Provider {
  pub fn new(root: &Root) -> Provider {
    Provider {
      root: root
    }
  }
}
