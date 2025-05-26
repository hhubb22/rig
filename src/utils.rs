// src/utils.rs
use anyhow::{Context, Result, bail};
use std::fs;
use std::io::{self, Write};
use std::path::Path;
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