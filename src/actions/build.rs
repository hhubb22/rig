// src/actions/build.rs
use anyhow::{bail, Context, Result};
use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command as OsCommand; // Renamed to avoid conflict

// Helper function to find project root.
// A more robust version would search upwards for CMakeLists.txt or a .rig-project file.
fn find_project_root() -> Result<PathBuf> {
    let current_dir = env::current_dir().context("Failed to get current directory")?;
    // For now, assume current directory is project root if CMakeLists.txt exists
    if current_dir.join("CMakeLists.txt").exists() {
        Ok(current_dir)
    } else {
        bail!("CMakeLists.txt not found in current directory or parent directories. Are you in a project?")
        // Future: Implement upward search
    }
}

// Helper to check if CMake configuration is needed
fn is_cmake_configured(project_root: &Path, preset_name: &str) -> Result<bool> {
    let build_dir = project_root.join("build").join(preset_name);
    // A simple check: if CMakeCache.txt exists in the build directory for the preset.
    // This isn't foolproof but is a common indicator.
    Ok(build_dir.join("CMakeCache.txt").exists())
}


pub fn build_project(preset_name: &str, clean_build: bool) -> Result<()> {
    let project_root = find_project_root()?;

    println!(
        "Building project at '{}' using preset '{}'...",
        project_root.display(),
        preset_name
    );

    let build_dir = project_root.join("build").join(preset_name);

    if clean_build && build_dir.exists() {
        println!("Cleaning build directory: {:?}", build_dir);
        fs::remove_dir_all(&build_dir)
            .with_context(|| format!("Failed to clean build directory: {:?}", build_dir))?;
    }

    // 1. Configure step (if not already configured, or if clean_build)
    //    CMake presets handle idempotency well, so re-configuring is usually fine.
    //    However, explicitly checking can save a bit of time if not needed.
    if clean_build || !is_cmake_configured(&project_root, preset_name)? {
        println!("Configuring CMake with preset '{}'...", preset_name);
        let configure_status = OsCommand::new("cmake")
            .arg("--preset")
            .arg(preset_name)
            .current_dir(&project_root)
            .status()
            .context("Failed to execute cmake configure command.")?;

        if !configure_status.success() {
            bail!(
                "CMake configuration failed for preset '{}' with exit code: {:?}",
                preset_name,
                configure_status.code()
            );
        }
        println!("CMake configuration successful.");
    } else {
        println!("CMake already configured for preset '{}'. Skipping configuration.", preset_name);
    }


    // 2. Build step
    println!("Building with CMake using preset '{}'...", preset_name);
    let build_status = OsCommand::new("cmake")
        .arg("--build")
        // .arg("--preset") // Note: Some older CMake versions might prefer build_dir path directly
        // .arg(preset_name) // For newer CMake: cmake --build --preset <name>
        .arg(build_dir) // More compatible: cmake --build <build_dir>
        // If you want to pass specific build args like -j
        // .arg("--")
        // .arg("-j")
        // .arg(num_cpus::get().to_string()) // Example: use all cores
        .current_dir(&project_root) // Not strictly necessary if using build_dir, but good practice
        .status()
        .context("Failed to execute cmake build command.")?;

    if !build_status.success() {
        bail!(
            "CMake build failed for preset '{}' with exit code: {:?}",
            preset_name,
            build_status.code()
        );
    }

    println!("Build successful for preset '{}'.", preset_name);
    Ok(())
}