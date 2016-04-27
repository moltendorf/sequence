pub mod application;
pub mod database;
pub mod daemon;
pub mod provider;
pub mod settings;

use self::application::Application;
use self::daemon::Daemon;
use self::database::Database;
use self::provider::Provider;
use self::settings::Settings;

pub struct Root<'a> {
  application: Application<'a>,
  daemon: Daemon,
  database: Database,
  provider: Provider<'a>,
  settings: Settings
}

impl<'a> Root<'a> {
  pub fn new<'b>() -> Root<'b> {
    let settings = Settings::new();
    let database = Database::new(&settings);
    let provider = Provider::new(&database);
    let application = Application::new(&provider);
    let daemon = Daemon::new(&settings, &provider);

    Root {
      application: application,
      daemon: daemon,
      database: database,
      provider: provider,
      settings: settings
    }
  }

  pub fn application(&self) -> &Application {
    &self.application
  }

  pub fn database(&self) -> &Database {
    &self.database
  }

  pub fn daemon(&self) -> &Daemon {
    &self.daemon
  }

  pub fn provider(&self) -> &Provider {
    &self.provider
  }

  pub fn settings(&self) -> &Settings {
    &self.settings
  }
}
