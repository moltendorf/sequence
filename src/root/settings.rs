extern crate toml;

use self::toml::Value;

use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::path::PathBuf;

pub struct Settings {
  home: PathBuf,
  table: Value
}

impl Settings {
  pub fn new() -> Settings {
    let mut path = env::home_dir().expect("No home environment variable found");
    let home = path.clone();

    path.push(".sequence");
    path.push("settings.toml");

    println!("Opening settings \"{}\"", path.to_string_lossy());

    let mut file = File::open(&path).expect("Could not find settings");
    let mut input = String::new();

    file.read_to_string(&mut input).expect("Could not read settings");

    Settings {
      home: home,
      table: input.parse().expect("Could not parse settings")
    }
  }

  pub fn lookup<'a>(&'a self, key: &'a str) -> Option<&Value> {
    self.table.lookup(key)
  }

  pub fn contains_key<'a>(&'a self, key: &'a str) -> bool {
    match self.table.lookup(key) {
      Some(_) => true,
      None => false
    }
  }

  pub fn home(&self) -> &PathBuf {
    &self.home
  }
}
