pub fn get_home_file_content(module_name: &str) -> String {
  let template = include_str!("home_template.txt");

  let replaced = template.replace("{}", module_name);

  replaced
}