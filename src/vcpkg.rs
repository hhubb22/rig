// src/vcpkg.rs
use crate::config::ProjectConfig;
use crate::utils::run_command;
use anyhow::{bail, Context, Result};
use std::env;
use std::path::PathBuf;

pub(crate) const VCPKG_JSON_FILENAME: &str = "vcpkg.json"; // Used by project.rs

pub struct VcpkgPaths {
    pub root: PathBuf,
    pub exe: PathBuf,
    #[allow(dead_code)]
    pub toolchain: PathBuf,
}

pub fn locate_and_verify_vcpkg(vcpkg_root_override: Option<String>) -> Result<VcpkgPaths> {
    let vcpkg_root_path_str = match vcpkg_root_override {
        Some(path) => path,
        None => env::var("VCPKG_ROOT")
            .context("VCPKG_ROOT environment variable not set and --vcpkg-root not provided.")?,
    };
    let vcpkg_root_path = PathBuf::from(&vcpkg_root_path_str);

    let vcpkg_exe_name = if cfg!(windows) { "vcpkg.exe" } else { "vcpkg" };
    let vcpkg_exe_path = vcpkg_root_path.join(vcpkg_exe_name);
    let vcpkg_cmake_toolchain_path =
        vcpkg_root_path.join("scripts/buildsystems/vcpkg.cmake");

    if !vcpkg_exe_path.is_file() {
        bail!("vcpkg executable not found at: {:?}", vcpkg_exe_path);
    }
    if !vcpkg_cmake_toolchain_path.is_file() {
        bail!(
            "vcpkg.cmake toolchain file not found at: {:?}",
            vcpkg_cmake_toolchain_path
        );
    }
    Ok(VcpkgPaths {
        root: vcpkg_root_path,
        exe: vcpkg_exe_path,
        toolchain: vcpkg_cmake_toolchain_path,
    })
}

pub fn initialize_manifest_and_dependencies(config: &ProjectConfig) -> Result<()> {
    println!("Initializing vcpkg manifest ({})...", VCPKG_JSON_FILENAME);
    run_command(
        &config.vcpkg_paths.exe,
        &["new", "--application"],
        Some(&config.project_path),
    )
    .context("Failed to initialize vcpkg manifest")?;

    if !config.dependencies.is_empty() {
        println!("Adding dependencies: {:?}", config.dependencies);
        let mut add_args: Vec<&str> = vec!["add", "port"];
        // Need to convert String to &str for the slice
        let dep_strs: Vec<String> = config.dependencies.iter().cloned().collect();
        let dep_refs: Vec<&str> = dep_strs.iter().map(AsRef::as_ref).collect();

        add_args.extend_from_slice(&dep_refs);

        if add_args.len() > 2 { // Ensure there are actual dependencies to add
            run_command(&config.vcpkg_paths.exe, &add_args, Some(&config.project_path))
                .with_context(|| format!("Failed to add vcpkg dependencies: {:?}", config.dependencies))?;
        }
    }
    Ok(())
}