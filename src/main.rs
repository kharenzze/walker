use walker::{App, AppConfig, DynResult};

fn main() -> DynResult<()>{
  let mut args = std::env::args();
  args.next();
  let path = args.next().expect("Missing path");
  let config = AppConfig::new(path);
  App::run(config)
}
