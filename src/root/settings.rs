extern crate toml;

use std::env;
use std::fs::File;
use std::io::prelude::*;

pub fn load() -> Result<String, String> {
	let mut path = match env::home_dir() {
		Some(path) => path,
		None => return Err("No home environment variable found!".to_string())
	};

	path.push(".sequence");
	path.push("settings.toml");

	let mut file = match File::open(&path) {
		Ok(file) => file,
		Err(_) => return Err(format!("No settings file found: {}!", path.to_string_lossy())),
	};

	println!("Opening settings file: {}!", path.to_string_lossy());

	let mut input = String::new();

	match file.read_to_string(&mut input) {
		Ok(_) => (),
		Err(_) => return Err(format!("Could not read settings file: {}!", path.to_string_lossy()))
	};

	let table = toml::Parser::new(&input).parse();

	let address = table.as_ref().and_then(
		|t| t.get("http")).and_then(
		|v| v.as_table()).and_then(
		|t| t.get("address")).and_then(
		|v| v.as_str());

	let address = match address {
		Some(address) => address.to_string(),
		None => return Err(format!("Could not get address from settings file: {}!", path.to_string_lossy()))
	};

	Ok(address)
}
