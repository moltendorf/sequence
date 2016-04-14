pub mod daemon;
pub mod settings;

use self::daemon::Daemon;
use self::settings::Settings;

pub struct Root {
  daemon: Option<Daemon>,
  settings: Option<Settings>
}

impl Root {
  pub fn new() -> Root {
    let mut root = Root {
      daemon: None,
      settings: None
    };

    root.settings = Some(Settings::new(&root));
    root.daemon = Some(Daemon::new(&root));

    root
  }

  pub fn daemon(&self) -> &Daemon {
    &self.daemon.as_ref().unwrap()
  }

  pub fn settings(&self) -> &Settings {
    &self.settings.as_ref().unwrap()
  }
}
