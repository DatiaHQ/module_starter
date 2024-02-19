use std::fs;

pub fn get_home_file_content(module_name: &str) -> String {
  let template = fs::read_to_string("home_template.txt")
      .expect("Could not read template file");

  let replaced = template.replace("{}", module_name);

  replaced
}