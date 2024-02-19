# Module Generator (IN_PROGRESS)

At Datia we have to continuously generate new modules/frameworks to improve our product and achieve our company goals. This project is a tool for automatically generating modules in Rust for our web application based in React.

## Requirements
- You need to have Rust and Cargo installed in your system.
- Copy and complete values for env vars:
```cp .env.example .env```

## Usage

To use this tool, you only need to provide the name of the module you want to generate. For example, if you want to generate a module called "EU Taxonomy", you can do so as follows:

```bash
cargo run EUTaxonomy
```
That's all! You will get your new module automatically added to the app, including a routing system with URL params, provider and components.