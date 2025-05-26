// src/actions/run.rs
use crate::actions::build::build_project; // Import the build function
use anyhow::{bail, Context, Result};
use std::path::PathBuf;
use std::process::Command as OsCommand;
use std::env;
use std::fs;


// find_project_details remains the same as before
fn find_project_details(target_override: Option<String>) -> Result<(PathBuf, String)> {
    let current_dir = env::current_dir().context("Failed to get current directory")?;
    let project_name = if let Some(name) = target_override {
        name
    } else {
        let vcpkg_json_path = current_dir.join("vcpkg.json");
        if vcpkg_json_path.exists() {
            let content = fs::read_to_string(&vcpkg_json_path)
                .with_context(|| format!("Failed to read vcpkg.json at {:?}", vcpkg_json_path))?;
            if let Some(line) = content.lines().find(|l| l.trim().starts_with(r#""name":"#)) {
                let name_part = line.split(':').nth(1).unwrap_or("").trim();
                let name = name_part.trim_matches(|c| c == '"' || c == ',').to_string();
                if !name.is_empty() { name } else {
                    current_dir.file_name().and_then(|name| name.to_str()).map(String::from)
                        .context("Failed to determine project name from current directory (vcpkg.json name empty)")?
                }
            } else {
                 current_dir.file_name().and_then(|name| name.to_str()).map(String::from)
                    .context("Failed to determine project name from current directory (vcpkg.json 'name' field not found)")?
            }
        } else {
            current_dir.file_name().and_then(|name| name.to_str()).map(String::from)
                .context("Failed to determine project name from current directory (vcpkg.json not found)")?
        }
    };
    Ok((current_dir, project_name))
}


pub fn run_project(
    preset: &str,
    target_override: Option<String>,
    clean_build_first: bool,
    args: &[String],
) -> Result<()> {
    // Step 1: Build the project
    println!("Ensuring project is built before running...");
    build_project(preset, clean_build_first)
        .with_context(|| format!("Build process for preset '{}' failed", preset))?;
    println!("Build check complete.");

    // Step 2: Proceed to run the executable (logic from previous run_project)
    let (project_root, project_name) = find_project_details(target_override)
        .context("Failed to find project details. Are you in a project directory?")?;

    println!(
        "Attempting to run target '{}' from project at '{}' using preset '{}'...",
        project_name,
        project_root.display(),
        preset
    );

    let mut exe_path = project_root.join("build").join(preset).join(&project_name);
    if cfg!(windows) {
        exe_path.set_extension("exe");
    }

    if !exe_path.exists() {
        // This should ideally not be hit if build_project succeeded and target name is correct
        bail!(
            "Executable not found at '{}' even after build. Check target name and CMakeLists.txt.",
            exe_path.display()
        );
    }

    println!("Executing: {} {}", exe_path.display(), args.join(" "));

    let mut command = OsCommand::new(&exe_path);
    command.args(args);
    command.current_dir(&project_root);

    let status = command
        .status()
        .with_context(|| format!("Failed to execute command: {}", exe_path.display()))?;

    if !status.success() {
        if let Some(code) = status.code() {
            eprintln!("Command exited with status: {}", code);
            std::process::exit(code);
        } else {
            bail!("Command terminated by signal");
        }
    }

    Ok(())
}