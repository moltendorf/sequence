pub mod httpd;
pub mod settings;

use self::settings::Settings;

pub struct Root {
  settings: Settings
}

impl Root {
  pub fn new() -> Root {
    Root {
      settings: Settings::new()
    }
  }

  pub fn settings(&self) -> &Settings {
    &self.settings
  }
}
