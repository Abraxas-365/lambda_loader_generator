mod creator;
extern crate handlebars;
extern crate serde;
extern crate serde_yaml;
use std::process::{exit, Command};

use std::fs;

use crate::creator::{generate_structure, initialize_go_project, populate_framework, GoStruct};

fn main() {
    let yaml_str = fs::read_to_string("structs.yaml").expect("Unable to read the file");
    let go_struct: GoStruct = serde_yaml::from_str(&yaml_str).expect("Unable to parse the YAML");

    if let Err(err) = initialize_go_project(&go_struct.project_name) {
        eprintln!("Error initializing Go project: {}", err);
        exit(1);
    }

    populate_framework(&go_struct.project_name);
    generate_structure(&go_struct.name, &go_struct.fields, &go_struct.project_name);

    if let Err(err) = Command::new("go")
        .args(&["mod", "tidy", &go_struct.project_name])
        .current_dir(&go_struct.project_name)
        .output()
    {
        eprintln!("Error running tidy project: {}", err);
        exit(1)
    }

    println!(
        "Go code has been generated and saved to {}",
        &go_struct.project_name
    );
}
