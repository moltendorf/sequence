extern crate mysql;

use super::super::Root;

use self::mysql::value::Value;

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Weak;

pub struct Preferences {
  root: Weak<RefCell<Root>>,
  table: String,
  map: RefCell<HashMap<String, String>>
}

impl Preferences {
  pub fn new(reference: Weak<RefCell<Root>>) -> Preferences {
    let strong = reference.upgrade().unwrap();
    let root = strong.borrow();
    let table = root.database().prefix("preferences");

    let preferences = Preferences {
      table: table,
      root: reference,
      map: RefCell::new(HashMap::new())
    };

    preferences.load();

    preferences
  }

  pub fn get(self, key: &String) -> Option<String> {
    // @todo How to return a reference instead of a clone?
    match self.map.borrow().get(key) {
      Some(value) => Some(value.clone()),
      None => None
    }
  }

  fn load(&self) {
    let strong = &self.root.upgrade().unwrap();
    let root = strong.borrow();
    let pool = root.database().pool();

    let mut map = self.map.borrow_mut();

    // @todo Figure out what the heck is going on here. I just hacked an example and made it work.
    let r: Vec<()> = pool.prep_exec(sql_select_preferences(&self.table), ()).map(
      |result| result.map(|x| x.unwrap()).map(|row| {
        let (preference_key, preference_value) = mysql::from_row(row);

        map.insert(preference_key, preference_value);
      }).collect()
    ).unwrap();
  }
}

fn sql_select_preferences(table: &String) -> String {
  format!(
  "SELECT preference_key, preference_value
   FROM {}",
  &table)
}
