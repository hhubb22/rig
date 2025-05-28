// src/actions/clean.rs
use anyhow::{Context, Result, bail};
use std::fs;
use crate::utils::find_project_root_by_marker;
use crate::cmake::CMAKELISTS_FILENAME; // Using CMakeLists.txt as a common project marker

pub fn clean_project(preset: Option<String>, all: bool) -> Result<()> {
    let project_root = find_project_root_by_marker(CMAKELISTS_FILENAME)
        .context("Failed to find project root (CMakeLists.txt). Are you in a CMake project?")?;

    let build_dir_base = project_root.join("build");

    if !build_dir_base.exists() || !build_dir_base.is_dir() {
        println!("Build directory '{}' does not exist or is not a directory. Nothing to clean.", build_dir_base.display());
        return Ok(());
    }

    if all {
        println!("Cleaning all build artifacts in '{}'...", build_dir_base.display());
        fs::remove_dir_all(&build_dir_base)
            .with_context(|| format!("Failed to remove directory: {:?}", build_dir_base))?;
        println!("Successfully cleaned all build artifacts.");
    } else if let Some(p_name) = preset {
        let preset_build_dir = build_dir_base.join(&p_name);
        if preset_build_dir.exists() {
            println!("Cleaning build artifacts for preset '{}' in '{}'...", p_name, preset_build_dir.display());
            fs::remove_dir_all(&preset_build_dir)
                .with_context(|| format!("Failed to remove directory: {:?}", preset_build_dir))?;
            println!("Successfully cleaned build artifacts for preset '{}'.", p_name);
        } else {
            println!("Build directory for preset '{}' ('{}') does not exist. Nothing to clean.", p_name, preset_build_dir.display());
        }
    } else {
        // This case should ideally not be reached if clap group is required.
        // If it can be reached, we might want to default to cleaning 'dev' or error.
        bail!("Please specify a preset to clean or use the --all flag. e.g., `rig clean --preset dev` or `rig clean --all`");
    }

    Ok(())
}