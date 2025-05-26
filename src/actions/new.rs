// src/actions/new.rs
use crate::cmake::{
    self, CMAKELISTS_FILENAME, CMAKE_PRESETS_FILENAME, CMAKE_USER_PRESETS_FILENAME,
};
#[allow(unused_imports)]
use crate::config::{ProjectConfig, GITIGNORE_FILENAME, MAIN_CPP_FILENAME};
use crate::utils::{handle_project_directory_creation, write_file_content};
use crate::vcpkg;
use anyhow::{Context, Result};
use std::fs;

// Helper function moved from project.rs and made private
// It now takes project_name as an argument to customize the output
fn generate_main_cpp_content(project_name: &str) -> String {
    format!(
        r#"#include <iostream>

// If you added "fmt" as a dependency, uncomment the line below
// and the fmt::print line in main():
// #include <fmt/core.h>

int main(int argc, char* argv[]) {{
    // fmt::print("Hello from {{}}!\n", "{}");
    std::cout << "Hello from {}!" << std::endl;
    if (argc > 1) {{
        std::cout << "Provided arguments:" << std::endl;
        for (int i = 1; i < argc; ++i) {{
            std::cout << i << ": " << argv[i] << std::endl;
        }}
    }}
    return 0;
}}
"#,
        project_name, // For fmt::print
        project_name  // For std::cout
    )
}

// Helper function moved from project.rs and made private
fn generate_gitignore_content() -> String {
    r#"# CMake
build/
install/
CMakeUserPresets.json
CMakeCache.txt
CMakeFiles/
cmake_install.cmake
compile_commands.json

# vcpkg
vcpkg_installed/

# IDE specific
.vscode/
.idea/
*.suo
*.ntvs*
*.njsproj
*.sln.docstates

# Compiled Object files & Precompiled Headers
*.slo
*.lo
*.o
*.obj
*.gch
*.pch

# Compiled Libraries & Executables
*.so
*.dylib
*.dll
*.lai
*.la
*.a
*.lib
*.exe
*.out
*.app

# Fortran module files
*.mod
*.smod
"#
    .to_string()
}

// Helper function moved from project.rs and made private
fn print_next_steps(config: &ProjectConfig) -> Result<()> {
    println!("\nProject '{}' created successfully!", config.project_name);
    println!(
        "  Path: {:?}",
        fs::canonicalize(&config.project_path).with_context(|| format!(
            "Failed to canonicalize project path: {:?}",
            config.project_path
        ))?
    );
    println!("\nNext steps:");
    println!("1. `cd {}`", config.project_name);
    println!("2. Configure: `cmake --preset dev`");
    println!("3. Build: `cmake --build --preset dev`");
    println!(
        "4. Run your executable (e.g., `./build/dev/{}` or `build\\dev\\{}.exe`)",
        config.project_name, config.project_name
    );
    println!("\nTo build for release (after `dev` preset used once):");
    println!("1. Configure: `cmake --preset release`");
    println!("2. Build: `cmake --build --preset release`");
    Ok(())
}

pub fn new_project(
    name: String,
    vcpkg_root_override: Option<String>,
    dependencies: Vec<String>,
    cpp_standard: String,
) -> Result<()> {
    let config = ProjectConfig::new(name, vcpkg_root_override, dependencies, cpp_standard)?;

    println!("Creating new C++ project: {}", config.project_name);
    println!("Using VCPKG_ROOT: {:?}", config.vcpkg_paths.root);

    handle_project_directory_creation(&config.project_path, &config.project_name)?;

    vcpkg::initialize_manifest_and_dependencies(&config)?;

    // Create CMakeLists.txt
    let cmakelists_content = cmake::generate_cmakelists_content(&config);
    write_file_content(
        &config.project_path.join(CMAKELISTS_FILENAME),
        &cmakelists_content,
    )?;

    // Create main.cpp (using the new project_name argument)
    let main_cpp_content = generate_main_cpp_content(&config.project_name);
    write_file_content(
        &config.project_path.join(&config.main_cpp_file), // main_cpp_file comes from config
        &main_cpp_content,
    )?;

    // Create CMakePresets.json
    let cmakepresets_content = cmake::generate_cmakepresets_content();
    write_file_content(
        &config.project_path.join(CMAKE_PRESETS_FILENAME),
        &cmakepresets_content,
    )?;

    // Create CMakeUserPresets.json
    let cmakeuserpresets_content =
        cmake::generate_cmakeuserpresets_content(&config.vcpkg_paths.root);
    write_file_content(
        &config.project_path.join(CMAKE_USER_PRESETS_FILENAME),
        &cmakeuserpresets_content,
    )?;

    // Create .gitignore
    let gitignore_content = generate_gitignore_content();
    write_file_content(
        &config.project_path.join(GITIGNORE_FILENAME),
        &gitignore_content,
    )?;

    print_next_steps(&config)?;

    Ok(())
}