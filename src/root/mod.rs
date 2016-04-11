pub mod httpd;
pub mod settings;

use self::settings::Settings;

struct Root {
	settings: Settings
}

impl Root {
	fn new() -> Root {
		Root {
			settings: Settings::new()
		}
	}
}
