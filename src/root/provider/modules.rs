extern crate mysql;

use super::super::Root;

use self::mysql::value::Value;

use std::cell::Ref;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Weak;

pub struct Modules {
  root: Weak<RefCell<Root>>,
  table: String,
  list: RefCell<Vec<String>>
}

impl Modules {
  pub fn new(reference: Weak<RefCell<Root>>) -> Modules {
    let strong = reference.upgrade().unwrap();
    let root = strong.borrow();
    let table = root.database().prefix("modules");

    let modules = Modules {
      table: table,
      root: reference,
      list: RefCell::new(Vec::new())
    };

    modules.load();

    modules
  }

  pub fn get<'a>(&'a self) -> Ref<Vec<String>> {
    self.list.borrow()
  }

  fn load(&self) {
    let strong = &self.root.upgrade().unwrap();
    let root = strong.borrow();
    let pool = root.database().pool();

    let mut list = self.list.borrow_mut();

    // @todo Figure out what the heck is going on here. I just hacked an example and made it work.
    let r: Vec<()> = pool.prep_exec(sql_select_modules(&self.table), ()).map(
      |result| result.map(|x| x.unwrap()).map(|row| {
        let module = mysql::from_row(row);

        list.push(module);
      }).collect()
    ).unwrap();
  }
}

fn sql_select_modules(table: &String) -> String {
  format!(
  "SELECT module_name
   FROM {}
   WHERE module_is_enabled = 1",
  &table)
}
