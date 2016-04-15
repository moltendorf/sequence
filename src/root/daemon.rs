extern crate iron;

use super::Root;

use self::iron::prelude::*;
use self::iron::status;

use std::net::SocketAddrV4;
use std::net::SocketAddrV6;

pub struct Daemon {
  settings: Settings
}

impl Daemon {
  pub fn new(root: &Root) -> Daemon {
    let settings = root.settings();

    let address = settings.lookup("daemon.address").expect("Could not find address in settings");
    let address = address.as_str().expect("Invalid type for address in settings").to_string();

    Daemon {
      settings: Settings {
        address: address
      }
    }
  }
}

impl Daemon {
  pub fn listen(&self) {
    let address = self.settings.address();

    println!("Opening iron server on \"{}\"", address);

    fn hello_world(_: &mut Request) -> IronResult<Response> {
      Ok(Response::with((status::Ok, "Hello Rust!")))
    }

    if let Ok(address) = address.parse::<SocketAddrV6>() {
      Iron::new(hello_world).http(address).unwrap();
    } else if let Ok(address) = address.parse::<SocketAddrV4>() {
      Iron::new(hello_world).http(address).unwrap();
    }

    panic!("Could not parse address");
  }

  pub fn settings(&self) -> &Settings {
    &self.settings
  }
}

pub struct Settings {
  address: String
}

impl Settings {
  pub fn address(&self) -> &String {
    &self.address
  }
}
