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

    let dir = Path::new(&core_repo_path).join("src").join("views").join("fund-manager").join(module_name);
    if !dir.exists() {
        fs::create_dir_all(&dir).expect("Error when creating directory");
        println!("Directory created: {:?}", dir);
    }

    let router_file_path = dir.join(format!("{}Router.js", module_name));
    fs::write(&router_file_path, "console.log('Router created!');\n").expect("Error when writing file");
    println!("File created: {:?}", router_file_path);

    let private_route_file_path = dir.join(format!("{}PrivateRoute.js", module_name));
    fs::write(&private_route_file_path, "console.log('PrivateRoute created!');\n").expect("Error when writing file");
    println!("File created: {:?}", private_route_file_path);

    let components_dir = dir.join("components");
    fs::create_dir_all(&components_dir).expect("Error when creating directory");
    println!("Directory created: {:?}", components_dir);

    let components_index_file_path = components_dir.join("index.js");
    fs::write(&components_index_file_path, "").expect("Error when writing file");
    println!("File created: {:?}", components_index_file_path);
}