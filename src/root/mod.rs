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

use std::rc::Rc;

pub struct Root {
  application: Option<Application>,
  daemon: Option<Daemon>,
  database: Option<Database>,
  provider: Option<Provider>,
  settings: Option<Settings>
}

impl Root {
  pub fn new() -> Rc<Root> {
    let mut root = Root {
      application: None,
      daemon: None,
      database: None,
      provider: None,
      settings: None
    };

    let reference = Rc::new(root);

    root.settings = Some(Settings::new(Rc::downgrade(&reference)));
    root.database = Some(Database::new(Rc::downgrade(&reference)));
    root.provider = Some(Provider::new(Rc::downgrade(&reference)));
    root.application = Some(Application::new(Rc::downgrade(&reference)));
    root.daemon = Some(Daemon::new(Rc::downgrade(&reference)));

    reference
  }

  pub fn application(&self) -> &Application {
    &self.application.as_ref().unwrap()
  }

  pub fn database(&self) -> &Database {
    &self.database.as_ref().unwrap()
  }

  pub fn daemon(&self) -> &Daemon {
    &self.daemon.as_ref().unwrap()
  }

  pub fn provider(&self) -> &Provider {
    &self.provider.as_ref().unwrap()
  }

  pub fn settings(&self) -> &Settings {
    &self.settings.as_ref().unwrap()
  }
}
