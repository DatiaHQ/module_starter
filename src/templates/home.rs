
pub fn get_home_file_content(module_name: &str) -> String {
  format!(r#"import React, {{ useEffect }} from "react";
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
"#, module_name, module_name, module_name)
}