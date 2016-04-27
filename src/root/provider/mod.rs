use super::database::Database;

pub struct Provider<'a> {
  database: &'a Database
}

impl<'a> Provider<'a> {
  pub fn new<'b>(database: &'b Database) -> Provider<'b> {
    Provider {
      database: database
    }
  }
}
