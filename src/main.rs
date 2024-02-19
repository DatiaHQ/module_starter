use std::env;
use dotenv;
mod templates {
  pub mod router;
  pub mod home;
}
mod utils;

use utils::{create_directory, create_router_file, create_private_route_file, create_sub_directory, create_file, create_home_file};

use clap::{App, Arg};


fn main() {
  dotenv::from_filename(".env").ok();

  let core_repo_path = env::var("CORE_REPO_PATH").expect("CORE_REPO_PATH is not defined");

  let matches = App::new("module_starter")
    .arg(
      Arg::with_name("module_name")
        .help("The name of the module")
        .required(true)
        .index(1),
    )
    .get_matches();

  let module_name = matches.value_of("module_name").unwrap();

  let dir = create_directory(&core_repo_path, &module_name);

  create_router_file(&dir, &module_name);
  create_private_route_file(&dir, &module_name);

  let components_dir = create_sub_directory(&dir, "components");
  create_file(&components_dir, "index.js", "");

  let views_dir = create_sub_directory(&dir, "views");
  create_file(&views_dir, "index.js", &format!("export {{ default as {}Home }} from \"./{}Home\";\n", module_name, module_name));

  let home_dir = create_sub_directory(&views_dir, &format!("{}Home", module_name));
  create_file(&home_dir, "index.js", &format!("export {{ default }} from \"./{}Home\";\n", module_name));
  create_home_file(&home_dir, &module_name);
}