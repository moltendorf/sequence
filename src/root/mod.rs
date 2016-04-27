pub mod database;
pub mod daemon;
pub mod settings;

use self::daemon::Daemon;
use self::database::Database;
use self::settings::Settings;

pub struct Root {
  daemon: Option<Daemon>,
  database: Option<Database>,
  settings: Option<Settings>
}

impl Root {
  pub fn new() -> Root {
    let mut root = Root {
      daemon: None,
      database: None,
      settings: None
    };

    root.settings = Some(Settings::new(&root));
    root.database = Some(Database::new(&root));
    root.daemon = Some(Daemon::new(&root));

    root
  }

  pub fn database(&self) -> &Database {
    &self.database.as_ref().unwrap()
  }

  pub fn daemon(&self) -> &Daemon {
    &self.daemon.as_ref().unwrap()
  }

  pub fn settings(&self) -> &Settings {
    &self.settings.as_ref().unwrap()
  }
}
