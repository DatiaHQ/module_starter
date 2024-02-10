# Module Generator (IN_PROGRESS)

At Datia we have to continuously generate new modules/frameworks to improve our product and achieve our company goals. This project is a tool for automatically generating modules in Rust for our web application based in React.

## Requirements
- You need to have Rust and Cargo installed in your system.
- Clone this repository and add the path where you have located our UI Repository by this `CORE_REPO_PATH={YOUR_UI_REPO_PATH}`

## Usage

To use this tool, you only need to provide the name of the module you want to generate. For example, if you want to generate a module called "EU Taxonomy", you can do so as follows:

```bash
cargo run
```
After you execute the above command, you will be asked to write the module name, in this case, you should enter: "EUTaxonomy", if you want to add a new module called "Temperature Score", you should enter: "TemperatureScore" and so on. That's all! You will get your new module automatically added to the app, including a routing system with URL params, provider and components.