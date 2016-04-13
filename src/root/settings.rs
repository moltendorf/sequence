extern crate toml;

use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::path::PathBuf;

pub struct Settings {
	address: String,
	home: PathBuf
}

impl Settings {
	pub fn new() -> Settings {
		let mut path = env::home_dir().expect("No home environment variable found!");
		let home = path.clone();

		path.push(".sequence");
		path.push("settings.toml");

		println!("Opening settings: \"{}\"!", path.to_string_lossy());

		let mut file = File::open(&path).expect("Could not find settings!");
		let mut input = String::new();

		file.read_to_string(&mut input).expect("Could not read settings!");

		let table: toml::Value = input.parse().expect("Could not parse settings!");
		let address = table.lookup("httpd.address").expect("Could not find address in settings!");
		let address = address.as_str().expect("Invalid type for address in settings!").to_string();

		Settings {
			address: address,
			home: home
		}
	}

	pub fn address(&self) -> &String {
		&self.address
	}

	pub fn home(&self) -> &PathBuf {
		&self.home
	}
}