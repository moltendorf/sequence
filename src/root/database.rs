extern crate mysql;

use super::Root;

use self::mysql::Opts;
use self::mysql::OptsBuilder;
use self::mysql::Pool;

use std::cell::RefCell;
use std::rc::Weak;

pub struct Database {
  pool: Pool
}

impl Database {
  pub fn new(root: Weak<RefCell<Root>>) -> Database {
    let strong = root.upgrade().unwrap();
    let root = strong.borrow();
    let settings = root.settings();

    let mut builder = OptsBuilder::new();

    let method: String;

    if let Some(socket) = settings.lookup("database.socket").and_then(|v| v.as_str()) {
      builder.unix_addr(Some(socket));

      method = socket.to_string();
    } else {
      // Can be optional.
      let host = settings.lookup("database.host").and_then(|v| v.as_str().map(|v| v.to_string())).unwrap_or("[::]".to_string());

      // Must exist.
      let port = settings.lookup("database.port").and_then(|v| v.as_integer()).unwrap_or(3306) as u16;

      method = format!("{}:{}", host, port);

      builder.ip_or_hostname(Some(host)).tcp_port(port);
    }

    // Can be optional.
    let username = settings.lookup("database.username").and_then(|v| v.as_str());
    let password = settings.lookup("database.password").and_then(|v| v.as_str());
    let database = settings.lookup("database.database").and_then(|v| v.as_str());

    builder.user(username).pass(password).db_name(database);

    println!("Opening mysql connection on \"{}\"", method);

    Database {
      pool: Pool::new(Opts::from(builder)).unwrap()
    }
  }

  pub fn pool(&self) -> &Pool {
    &self.pool
  }
}
