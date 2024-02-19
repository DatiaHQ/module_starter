use heck::{ShoutySnakeCase, KebabCase, TitleCase};

pub fn get_router_file_content(module_name: &str) -> String {
  let module_name_upper_snake = module_name.to_shouty_snake_case();
  let module_name_kebab = module_name.to_kebab_case();
  let module_name_sentence = module_name.to_title_case().replace("_", " ");

  let template = include_str!("router_template.txt");

  let replaced = template
    .replace("{module_name_upper_snake}", &module_name_upper_snake)
    .replace("{module_name}", module_name)
    .replace("{module_name_kebab}", &module_name_kebab)
    .replace("{module_name_sentence}", &module_name_sentence);

  replaced
}