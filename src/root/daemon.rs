extern crate iron;

use super::settings::Settings;
use super::provider::Provider;

use self::iron::prelude::*;
use self::iron::status;

use std::net::SocketAddrV4;
use std::net::SocketAddrV6;

pub struct Daemon {
  address: String
}

impl Daemon {
  pub fn new(settings: &Settings, provider: &Provider) -> Daemon {
    let address = settings.lookup("daemon.address").expect("Could not find address in settings");
    let address = address.as_str().expect("Invalid type for address in settings").to_string();

    Daemon {
      address: address
    }
  }

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
}
