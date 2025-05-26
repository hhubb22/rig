// src/utils.rs
use std::env;
use anyhow::{Context, Result, bail};
use std::fs;
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::process::Command as OsCommand;

pub fn write_file_content(path: &Path, content: &str) -> Result<()> {
    let mut file = fs::File::create(path)
        .with_context(|| format!("Failed to create file: {:?}", path))?;
    file.write_all(content.as_bytes())
        .with_context(|| format!("Failed to write to file: {:?}", path))?;
    println!("Created file: {:?}", path);
    Ok(())
}

pub fn run_command(
    command_path: &Path,
    args: &[&str],
    current_dir: Option<&Path>,
) -> Result<()> {
    let mut cmd_instance = OsCommand::new(command_path);
    cmd_instance.args(args);
    if let Some(dir) = current_dir {
        cmd_instance.current_dir(dir);
    }

    let cmd_desc = format!(
        "{} {}",
        command_path.file_name().unwrap_or_default().to_string_lossy(),
        args.join(" ")
    );
    let dir_desc = current_dir.unwrap_or_else(|| Path::new("."));

    println!("Executing: {} (in {:?})", cmd_desc, dir_desc);

    let status = cmd_instance.status().with_context(|| {
        format!(
            "Failed to execute command: {} in {:?}",
            cmd_desc, dir_desc
        )
    })?;

    if !status.success() {
        bail!(
            "Command failed: {} (in {:?}) (exit code: {:?})",
            cmd_desc,
            dir_desc,
            status.code()
        );
    }
    Ok(())
}

pub fn handle_project_directory_creation(project_path: &Path, project_name: &str) -> Result<()> {
    if project_path.exists() {
        print!(
            "Directory '{}' already exists. Overwrite? (y/N): ",
            project_name
        );
        io::stdout().flush().context("Failed to flush stdout")?;
        let mut response = String::new();
        io::stdin()
            .read_line(&mut response)
            .context("Failed to read user input")?;
        if !response.trim().eq_ignore_ascii_case("y") {
            println!("Aborted.");
            bail!("User aborted project creation.");
        }
        fs::remove_dir_all(project_path)
            .with_context(|| format!("Failed to remove existing directory: {:?}", project_path))?;
    }
    fs::create_dir_all(project_path)
        .with_context(|| format!("Failed to create project directory: {:?}", project_path))?;
    println!("Created directory: {:?}", project_path);
    Ok(())
}

/// Searches upwards from the current directory for a specific marker file or directory.
pub fn find_project_root_by_marker(marker_filename: &str) -> Result<PathBuf> {
    let mut current_dir = env::current_dir().context("Failed to get current directory")?;
    loop {
        if current_dir.join(marker_filename).exists() {
            return Ok(current_dir);
        }
        match current_dir.parent() {
            Some(parent) => current_dir = parent.to_path_buf(),
            None => bail!(
                "Could not find project root: '{}' not found in current directory or any parent.",
                marker_filename
            ),
        }
    }
}