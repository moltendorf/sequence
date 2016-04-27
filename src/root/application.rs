use super::provider::Provider;

pub struct Application<'a> {
  provider: &'a Provider<'a>
}

impl<'a> Application<'a> {
  pub fn new<'b>(provider: &'b Provider) -> Application<'b> {
    Application {
      provider: provider
    }
  }
}
