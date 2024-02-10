use std::fs;
use std::io::{stdin, stdout, Write};
use std::path::Path;
use std::env;
use dotenv;
use heck::ShoutySnakeCase;
use heck::KebabCase;
use heck::TitleCase;

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

		let module_name_upper_snake = module_name.to_shouty_snake_case();

    let module_name_kebab = module_name.to_kebab_case();

    let module_name_sentence = module_name.to_title_case().replace("_", " ");

    let router_file_path = dir.join(format!("{}Router.js", module_name));
    let router_file_content = format!(r#"import React from "react";
import {{ Route, Switch, Redirect }} from "react-router-dom";
import {{ FUND_MANAGER_FUND_{}_ROUTE }} from "routes";
import {{ useFund }} from "providers/fund-manager/FundProvider";
import {{ PageTitle }} from "components";
import {{ {}Provider }} from "providers/fund-manager/{}";
import {{ {}Home }} from "./views";
import {}PrivateRoute from "./{}PrivateRoute";

const {}Router = () => {{
  const {{ data: {{ currentFund: fund }} = {{}} }} = useFund();
  return (
    <{}PrivateRoute>
      <PageTitle title={{`${{fund.name}} {}`}} />
      <{}Provider>
        <Switch>
          <Route
            render={{() => <{}Home />}}
            exact
            path={{FUND_MANAGER_FUND_{}_ROUTE}}
          />
          <Redirect to={{FUND_MANAGER_FUND_{}_ROUTE}} />
        </Switch>
      </{}Provider>
    </{}PrivateRoute>
  );
}};

export default {}Router;
"#, module_name_upper_snake, module_name, module_name_kebab, module_name, module_name, module_name, module_name, module_name, module_name_sentence, module_name, module_name, module_name_upper_snake, module_name_upper_snake, module_name, module_name, module_name);
    fs::write(&router_file_path, router_file_content).expect("Error when writing file");
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

    let views_dir = dir.join("views");
    fs::create_dir_all(&views_dir).expect("Error when creating directory");
    println!("Directory created: {:?}", views_dir);

    let views_index_file_path = views_dir.join("index.js");
    fs::write(&views_index_file_path, format!("export {{ default as {}Home }} from \"./{}Home\";\n", module_name, module_name)).expect("Error when writing file");
    println!("File created: {:?}", views_index_file_path);

    let home_dir = views_dir.join(format!("{}Home", module_name));
    fs::create_dir_all(&home_dir).expect("Error when creating directory");
    println!("Directory created: {:?}", home_dir);

    let home_index_file_path = home_dir.join("index.js");
    fs::write(&home_index_file_path, format!("export {{ default }} from \"./{}Home\";\n", module_name)).expect("Error when writing file");
    println!("File created: {:?}", home_index_file_path);

    let home_file_path = home_dir.join(format!("{}Home.js", module_name));
    let home_file_content = format!(r#"import React, {{ useEffect }} from "react";
import {{ useDefaultQueryParams }} from "utils";
import {{ useBreadcrumb }} from "providers/fund-manager/BreadcrumbProvider";

const {}Home = () => {{
  const {{
    actions: {{ updateBreadcrumb }},
  }} = useBreadcrumb();

  const defaultParams = {{}};

  useDefaultQueryParams(defaultParams, () => {{}}, []);

  useEffect(() => {{
    updateBreadcrumb();
  }}, []);

  return <div>{} Home</div>;
}};

export default {}Home;
"#, module_name, module_name, module_name);
    fs::write(&home_file_path, home_file_content).expect("Error when writing file");
    println!("File created: {:?}", home_file_path);
}