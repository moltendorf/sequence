extern crate iron;

extern crate core;
extern crate sequence;

use core::str::FromStr;

use std::net::SocketAddrV4;
use std::net::SocketAddrV6;

use iron::prelude::*;
use iron::status;

use sequence::root::Root;

fn main() {
	let root = Root::new();

	let address = root.settings().address();

	println!("Opening iron server on: {}!", address);

	fn hello_world(_: &mut Request) -> IronResult<Response> {
		Ok(Response::with((status::Ok, "Hello Rust!")))
	}

	match SocketAddrV4::from_str(&address) {
		Ok(socket) => {
			Iron::new(hello_world).http(socket).unwrap();
			()
		},
		Err(_) => match SocketAddrV6::from_str(&address) {
			Ok(socket) => {
				Iron::new(hello_world).http(socket).unwrap();
				()
			},
			Err(_) => panic!("Could not parse address: {}", address)
		}
	}
}
