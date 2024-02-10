use std::fs;
use std::io::{stdin, stdout, Write};
use std::path::Path;
use std::env;
use dotenv;

fn main() {
	dotenv::from_filename(".env").ok();

	let core_repo_path = env::var("CORE_REPO_PATH").expect("CORE_REPO_PATH is not defined");

	let mut module_name = String::new();
	print!("Type the module name (e.g: TemperatureScore): ");
	let _= stdout().flush();
	stdin().read_line(&mut module_name).expect("Error when reading the line");
	let module_name = module_name.trim();

	let dir = Path::new(&core_repo_path).join(module_name);
	if !dir.exists() {
		fs::create_dir_all(&dir).expect("Error when creating directory");
	}

	let file_path = dir.join("index.js");
	fs::write(file_path, "console.log('Module created!');\n").expect("Error when writing file");
}