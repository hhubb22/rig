// src/config.rs
use crate::vcpkg::{self, VcpkgPaths};
use anyhow::Result;
use std::path::PathBuf;

// Filename constants related to project structure
pub(crate) const MAIN_CPP_FILENAME: &str = "main.cc";
pub(crate) const GITIGNORE_FILENAME: &str = ".gitignore";

pub struct ProjectConfig {
    pub project_name: String,
    pub project_path: PathBuf,
    pub vcpkg_paths: VcpkgPaths,
    pub dependencies: Vec<String>,
    pub cpp_standard: String,
    // Add other common paths or settings here if needed
    pub main_cpp_file: String, // e.g. "main.cc"
}

impl ProjectConfig {
    pub fn new(
        project_name: String,
        vcpkg_root_override: Option<String>,
        dependencies: Vec<String>,
        cpp_standard: String,
    ) -> Result<Self> {
        let vcpkg_paths = vcpkg::locate_and_verify_vcpkg(vcpkg_root_override)?;
        let project_path = PathBuf::from(&project_name);

        Ok(Self {
            project_name,
            project_path,
            vcpkg_paths,
            dependencies,
            cpp_standard,
            main_cpp_file: MAIN_CPP_FILENAME.to_string(),
        })
    }
}