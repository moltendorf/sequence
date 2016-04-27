use super::database::Database;

pub struct Provider {
  database: &Database
}

impl Provider {
  pub fn new(database: &Database) -> Provider {
    Provider {
      database: database
    }
  }
}
