use heck::{ShoutySnakeCase, KebabCase, TitleCase};

pub fn get_router_file_content(module_name: &str) -> String {
  let module_name_upper_snake = module_name.to_shouty_snake_case();

  let module_name_kebab = module_name.to_kebab_case();

  let module_name_sentence = module_name.to_title_case().replace("_", " ");

  format!(r#"import React from "react";
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
"#, module_name_upper_snake, module_name, module_name_kebab, module_name, module_name, module_name, module_name, module_name, module_name_sentence, module_name, module_name, module_name_upper_snake, module_name_upper_snake, module_name, module_name, module_name)
}