extern crate iron;
extern crate toml;

use std::env;
use std::fs::File;
use std::io::prelude::*;

use iron::prelude::*;
use iron::status;

fn main() {
	let mut path = env::home_dir().unwrap();

	path.push(".sequence");
	path.push("settings.toml");

	let mut file = match File::open(&path) {
		Ok(file) => file,
		Err(_) => panic!("No settings file found: {}!", path.to_string_lossy()),
	};

	println!("Opening settings file: {}!", path.to_string_lossy());

	let mut input = String::new();
	file.read_to_string(&mut input).unwrap();

	let value = toml::Parser::new(&input).parse().unwrap();
	println!("{:?}", value);

	fn hello_world(_: &mut Request) -> IronResult<Response> {
		Ok(Response::with((status::Ok, "Hello Rust!")))
	}

//	Iron::new(hello_world).http("[::1]:8001").unwrap();
}
