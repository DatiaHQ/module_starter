use std::fs;
use std::io::{stdin, stdout, Write};
use std::path::{Path, PathBuf};
use crate::templates::router;
use crate::templates::home;

pub fn get_module_name() -> String {
  let mut module_name = String::new();
  print!("Type the module name (e.g: TemperatureScore): ");
  let _= stdout().flush();
  stdin().read_line(&mut module_name).expect("Error when reading the line");
  module_name.trim().to_string()
}

pub fn create_directory(core_repo_path: &str, module_name: &str) -> PathBuf {
  let dir = Path::new(core_repo_path).join("src").join("views").join("fund-manager").join(module_name);
  if !dir.exists() {
    fs::create_dir_all(&dir).expect("Error when creating directory");
    println!("Directory created: {:?}", dir);
  }
  dir
}

pub fn create_router_file(dir: &Path, module_name: &str) {
  let router_file_path = dir.join(format!("{}Router.js", module_name));
  let router_file_content = router::get_router_file_content(module_name);
  fs::write(&router_file_path, router_file_content).expect("Error when writing file");
  println!("File created: {:?}", router_file_path);
}

pub fn create_private_route_file(dir: &Path, module_name: &str) {
  let private_route_file_path = dir.join(format!("{}PrivateRoute.js", module_name));
  fs::write(&private_route_file_path, "console.log('PrivateRoute created!');\n").expect("Error when writing file");
  println!("File created: {:?}", private_route_file_path);
}

pub fn create_sub_directory(dir: &Path, sub_dir_name: &str) -> PathBuf {
  let sub_dir = dir.join(sub_dir_name);
  fs::create_dir_all(&sub_dir).expect("Error when creating directory");
  println!("Directory created: {:?}", sub_dir);
  sub_dir
}

pub fn create_file(dir: &Path, file_name: &str, content: &str) {
  let file_path = dir.join(file_name);
  fs::write(&file_path, content).expect("Error when writing file");
  println!("File created: {:?}", file_path);
}

pub fn create_home_file(dir: &Path, module_name: &str) {
  let home_file_path = dir.join(format!("{}Home.js", module_name));
  let home_file_content = home::get_home_file_content(module_name);
  fs::write(&home_file_path, home_file_content).expect("Error when writing file");
  println!("File created: {:?}", home_file_path);
}