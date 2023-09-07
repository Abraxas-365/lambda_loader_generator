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

    println!(
        "Go code has been generated and saved to {}",
        &go_struct.project_name
    );
    let go_mod_tidy = Command::new("go")
        .args(&["mod", "tidy"])
        .current_dir(&go_struct.project_name)
        .output()
        .expect("Failed to run go mod tidy");

    if !go_mod_tidy.status.success() {
        eprintln!(
            "go mod tidy failed: {}",
            String::from_utf8_lossy(&go_mod_tidy.stderr)
        );
        exit(1);
    }

    println!("go mod tidy has been successfully executed.");
}
