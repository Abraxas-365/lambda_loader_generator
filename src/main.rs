mod creator;
extern crate clap;
extern crate handlebars;
extern crate serde;
extern crate serde_yaml;

use clap::{App, Arg};
use std::fs;
use std::process::{exit, Command};

use crate::creator::{generate_structure, initialize_go_project, populate_framework, GoStruct};

fn main() {
    let matches = App::new("Your CLI Tool")
        .version("1.0")
        .author("Your Name")
        .about("Does awesome things")
        .arg(
            Arg::with_name("yml_file")
                .short("y")
                .long("yml")
                .value_name("YML_FILE")
                .help("Sets the input YAML file")
                .required(true)
                .takes_value(true),
        )
        .arg(
            Arg::with_name("output_dir")
                .short("o")
                .long("output")
                .value_name("OUTPUT_DIR")
                .help("Sets the output directory for the generated Go project")
                .required(true)
                .takes_value(true),
        )
        .get_matches();

    let yml_file = matches.value_of("yml_file").unwrap();
    let output_dir = matches.value_of("output_dir").unwrap();

    if !check_go() {
        eprintln!("Go is not installed. Please install Go and try again.");
        exit(1);
    }

    let yaml_str = fs::read_to_string(yml_file).expect("Unable to read the file");
    let go_struct: GoStruct = serde_yaml::from_str(&yaml_str).expect("Unable to parse the YAML");
    if let Err(err) = go_struct.validate() {
        eprintln!("Validation error: {}", err);
        exit(1);
    }

    if let Err(err) = initialize_go_project(output_dir) {
        eprintln!("Error initializing Go project: {}", err);
        exit(1);
    }

    populate_framework(output_dir);
    generate_structure(&go_struct.name, &go_struct.fields, output_dir);

    println!("Go code has been generated and saved to {}", output_dir);

    let go_mod_tidy = Command::new("go")
        .args(&["mod", "tidy"])
        .current_dir(output_dir)
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

fn check_go() -> bool {
    let check_go = Command::new("go")
        .arg("version")
        .output()
        .expect("Failed to execute command");

    if !check_go.status.success() {
        return false;
    }
    true
}
