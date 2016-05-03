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

use std::cell::RefCell;
use std::rc::Rc;

pub struct Root {
  application: Option<Application>,
  daemon: Option<Daemon>,
  database: Option<Database>,
  provider: Option<Provider>,
  settings: Option<Settings>
}

impl Root {
  pub fn new() -> Rc<RefCell<Root>> {
    let mut strong = Rc::new(RefCell::new(Root {
      application: None,
      daemon: None,
      database: None,
      provider: None,
      settings: None
    }));

    let weak = Rc::downgrade(&strong);

    let settings = Some(Settings::new(weak.clone()));
    strong.borrow_mut().settings = settings;

    let database = Some(Database::new(weak.clone()));
    strong.borrow_mut().database = database;

    let provider = Some(Provider::new(weak.clone()));
    strong.borrow_mut().provider = provider;

    let application = Some(Application::new(weak.clone()));
    strong.borrow_mut().application = application;

    let daemon = Some(Daemon::new(weak.clone()));
    strong.borrow_mut().daemon = daemon;

    strong
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
