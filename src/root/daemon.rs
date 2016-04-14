use super::Root;

pub struct Daemon {
  settings: Settings
}

impl Daemon {
  pub fn new(root: &Root) -> Daemon {
    let settings = root.settings();

    let address = settings.lookup("daemon.address".to_string()).expect("Could not find address in settings");
    let address = address.as_str().expect("Invalid type for address in settings").to_string();

    Daemon {
      settings: Settings {
        address: address
      }
    }
  }
}

struct Settings {
  address: String
}

impl Settings {
  pub fn address(&self) -> &String {
    &self.address
  }
}
