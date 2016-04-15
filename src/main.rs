extern crate sequence;

use sequence::root::Root;

fn main() {
  let root = Root::new();

  root.daemon().listen().or_else(|message| panic!("{}", message));
}
