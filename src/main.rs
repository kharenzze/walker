use walker::{App, AppConfig};

fn main() {
  let mut args = std::env::args();
  args.next();
  let path = args.next().expect("Missing path");
  let config = AppConfig::new(path);
  App::run(config).expect("Something went wrong")
}
