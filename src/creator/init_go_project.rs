use std::{fs, process::Command};

pub fn initialize_go_project(project_name: &str) -> std::io::Result<()> {
    // Create the project directory
    fs::create_dir(project_name)?;

    // Initialize Go module
    Command::new("go")
        .args(&["mod", "init", project_name])
        .current_dir(project_name)
        .output()?;

    // Create project structure
    let sub_dirs = vec![
        "cmd",
        "internal/object",
        "pkg/awsobject",
        "pkg/config",
        "pkg/logger",
    ];
    for dir in sub_dirs {
        let full_path = format!("{}/{}", project_name, dir);
        fs::create_dir_all(full_path)?; // Note the change to create_dir_all here
    }

    Ok(())
}
