// src/actions/run.rs
use crate::actions::build::build_project;
use crate::utils::find_project_root_by_marker; // Import the new utility
use crate::vcpkg::VCPKG_JSON_FILENAME; // For marker
use anyhow::{bail, Context, Result};
use std::path::PathBuf;
use std::process::Command as OsCommand;
use std::fs;

// Modified find_project_name, assumes project_root is already found
fn determine_project_name(project_root: &PathBuf, target_override: Option<String>) -> Result<String> {
    if let Some(name) = target_override {
        return Ok(name);
    }

    let vcpkg_json_path = project_root.join(VCPKG_JSON_FILENAME);
    if vcpkg_json_path.exists() {
        let content = fs::read_to_string(&vcpkg_json_path)
            .with_context(|| format!("Failed to read {} at {:?}", VCPKG_JSON_FILENAME, vcpkg_json_path))?;
        if let Some(line) = content.lines().find(|l| l.trim().starts_with(r#""name":"#)) {
            let name_part = line.split(':').nth(1).unwrap_or("").trim();
            let name = name_part.trim_matches(|c| c == '"' || c == ',').to_string();
            if !name.is_empty() { return Ok(name); }
        }
    }
    // Fallback to directory name if not in vcpkg.json or vcpkg.json doesn't exist
    project_root.file_name().and_then(|name| name.to_str()).map(String::from)
        .context("Failed to determine project name from project directory")
}


pub fn run_project(
    preset: &str,
    target_override: Option<String>,
    clean_build_first: bool,
    args: &[String],
) -> Result<()> {
    println!("Ensuring project is built before running...");
    build_project(preset, clean_build_first)
        .with_context(|| format!("Build process for preset '{}' failed", preset))?;
    println!("Build check complete.");

    // Find project root
    let project_root = find_project_root_by_marker(VCPKG_JSON_FILENAME) // Or CMAKELISTS_FILENAME
        .context("Failed to find project root. Are you in a project directory?")?;
    
    // Determine project name
    let project_name = determine_project_name(&project_root, target_override)
        .context("Failed to determine project name.")?;


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
        bail!(
            "Executable not found at '{}'. Check target name and CMakeLists.txt.",
            exe_path.display()
        );
    }

    println!("Executing: {} {}", exe_path.display(), args.join(" "));

    let mut command = OsCommand::new(&exe_path);
    command.args(args);
    command.current_dir(&project_root); // Executing from project root is fine

    let status = command
        .status()
        .with_context(|| format!("Failed to execute command: {}", exe_path.display()))?;

    if !status.success() {
        if let Some(code) = status.code() {
            eprintln!("Command exited with status: {}", code);
            std::process::exit(code); // Exit with the same code
        } else {
            bail!("Command terminated by signal");
        }
    }

    Ok(())
}