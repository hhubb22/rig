use clap::{Parser, Subcommand};
use std::env;
use std::fs;
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::process::Command as OsCommand; // Alias to avoid conflict with our enum

#[derive(Parser)]
#[clap(author, version, about = "A CLI tool to create C++/CMake/vcpkg projects", long_about = None)]
struct Args {
    #[command(subcommand)]
    command: CliCommand,
}

#[derive(Subcommand)]
enum CliCommand {
    /// Creates a new C++ project with CMake and vcpkg
    New {
        /// Name of the new project
        name: String,

        /// Path to the vcpkg root directory.
        /// If not provided, uses the VCPKG_ROOT environment variable.
        #[clap(long)]
        vcpkg_root: Option<String>,

        /// List of vcpkg dependencies to add (e.g., "fmt", "spdlog")
        #[clap(long, value_delimiter = ',', default_value = "fmt")]
        deps: Vec<String>,
    },
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    match args.command {
        CliCommand::New {
            name,
            vcpkg_root,
            deps,
        } => {
            create_cpp_project(&name, vcpkg_root, &deps)?;
        }
    }
    Ok(())
}

fn create_cpp_project(
    project_name: &str,
    vcpkg_root_override: Option<String>,
    dependencies: &[String],
) -> Result<(), Box<dyn std::error::Error>> {
    println!("Creating new C++ project: {}", project_name);

    // 1. Determine and validate VCPKG_ROOT
    let vcpkg_root_path_str = match vcpkg_root_override {
        Some(path) => path,
        None => env::var("VCPKG_ROOT").map_err(
            |_| "VCPKG_ROOT environment variable not set and --vcpkg-root not provided.",
        )?,
    };
    let vcpkg_root_path = PathBuf::from(&vcpkg_root_path_str);

    let vcpkg_exe_name = if cfg!(windows) { "vcpkg.exe" } else { "vcpkg" };
    let vcpkg_exe_path = vcpkg_root_path.join(vcpkg_exe_name);
    let vcpkg_cmake_toolchain_path = vcpkg_root_path.join("scripts/buildsystems/vcpkg.cmake");

    if !vcpkg_exe_path.is_file() {
        return Err(format!("vcpkg executable not found at: {:?}", vcpkg_exe_path).into());
    }
    if !vcpkg_cmake_toolchain_path.is_file() {
        return Err(format!(
            "vcpkg.cmake toolchain file not found at: {:?}",
            vcpkg_cmake_toolchain_path
        )
        .into());
    }
    println!("Using VCPKG_ROOT: {:?}", vcpkg_root_path);

    // 2. Create project directory
    let project_path = PathBuf::from(project_name);
    if project_path.exists() {
        // Basic prompt for overwrite, consider making this a CLI flag
        print!(
            "Directory '{}' already exists. Overwrite? (y/N): ",
            project_name
        );
        io::stdout().flush()?;
        let mut response = String::new();
        io::stdin().read_line(&mut response)?;
        if !response.trim().eq_ignore_ascii_case("y") {
            println!("Aborted.");
            return Ok(());
        }
        fs::remove_dir_all(&project_path)?;
    }
    fs::create_dir_all(&project_path)?;
    println!("Created directory: {:?}", project_path);

    // 3. Initialize vcpkg manifest and add dependencies
    println!("Initializing vcpkg manifest (vcpkg.json)...");
    run_command(
        &vcpkg_exe_path,
        &["new", "--application"],
        Some(&project_path),
    )?;

    if !dependencies.is_empty() {
        println!("Adding dependencies: {:?}", dependencies);
        for dep in dependencies {
            if dep.is_empty() {
                continue;
            }
            println!("Adding port: {}", dep);
            run_command(&vcpkg_exe_path, &["add", "port", dep], Some(&project_path))?;
        }
    }
    // vcpkg new also creates vcpkg-configuration.json

    // 4. Create CMakeLists.txt
    let main_cpp_file = "main.cpp".to_string(); // Or format!("{}.cpp", project_name.to_lowercase());
    let cmakelists_content = format!(
        r#"cmake_minimum_required(VERSION 3.18)
project({0} CXX)

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CMAKE_EXPORT_COMPILE_COMMANDS ON)

# Vcpkg integration (FetchContent is an alternative for more complex scenarios)
# Ensure CMAKE_TOOLCHAIN_FILE is set, typically via CMakePresets.json or command line

find_package(fmt CONFIG REQUIRED) # Example, will be used by default main.cpp
# Add other find_package calls for your dependencies here, e.g.:
# find_package(spdlog CONFIG REQUIRED)

add_executable({0} {1})

target_link_libraries({0} PRIVATE fmt::fmt) # Example
# Add other target_link_libraries here, e.g.:
# target_link_libraries({0} PRIVATE spdlog::spdlog)

# Optional: Add include directories if vcpkg doesn't handle it automatically
# target_include_directories({0} PRIVATE ${{CMAKE_BINARY_DIR}}/vcpkg_installed/${{VCPKG_TARGET_TRIPLET}}/include)

# Example for tests using GTest (if you add gtest as a dependency)
# enable_testing()
# find_package(GTest CONFIG REQUIRED)
# add_executable(UnitTests tests/test_main.cpp)
# target_link_libraries(UnitTests PRIVATE GTest::gtest GTest::gtest_main {0})
# include(GoogleTest)
# gtest_discover_tests(UnitTests)
"#,
        project_name, main_cpp_file
    );
    write_file_content(&project_path.join("CMakeLists.txt"), &cmakelists_content)?;

    // 5. Create main.cpp
    let main_cpp_content = format!(
        r#"#include <fmt/core.h> // From vcpkg
#include <iostream>

int main(int argc, char* argv[]) {{
    fmt::print("Hello from {}!\\n", "{}");
    return 0;
}}
"#,
        project_name,
        project_name // First {} is project name, second is for fmt::print
    );
    write_file_content(&project_path.join(&main_cpp_file), &main_cpp_content)?;

    // 6. Create CMakePresets.json (content remains the same as before)
    let cmakepresets_content = r#"{
            "version": 3,
            "configurePresets": [
              {
                "name": "vcpkg-base",
                "hidden": true,
                "generator": "Ninja",
                "binaryDir": "${sourceDir}/build/${presetName}",
                "installDir": "${sourceDir}/install/${presetName}",
                "cacheVariables": {
                  "CMAKE_TOOLCHAIN_FILE": "$env{VCPKG_ROOT}/scripts/buildsystems/vcpkg.cmake",
                  "CMAKE_EXPORT_COMPILE_COMMANDS": "ON"
                }
              },
              {
                "name": "debug",
                "displayName": "Debug Build",
                "description": "Debug build with vcpkg integration",
                "inherits": "vcpkg-base",
                "cacheVariables": { "CMAKE_BUILD_TYPE": "Debug" }
              },
              {
                "name": "release",
                "displayName": "Release Build",
                "description": "Release build with vcpkg integration",
                "inherits": "vcpkg-base",
                "cacheVariables": { "CMAKE_BUILD_TYPE": "Release" }
              }
            ],
            "buildPresets": [
              { "name": "debug", "configurePreset": "debug", "displayName": "Build (Debug)" },
              { "name": "release", "configurePreset": "release", "displayName": "Build (Release)" }
            ],
            "testPresets": [
              { "name": "debug", "configurePreset": "debug", "output": {"outputOnFailure": true}, "execution": {"noTestsAction": "error", "stopOnFailure": true} },
              { "name": "release", "configurePreset": "release", "output": {"outputOnFailure": true}, "execution": {"noTestsAction": "error", "stopOnFailure": true} }
            ]
          }"#;
    write_file_content(
        &project_path.join("CMakePresets.json"),
        cmakepresets_content,
    )?;

    // 7. Create CMakeUserPresets.json
    let vcpkg_root_json_escaped = vcpkg_root_path.to_string_lossy().replace('\\', "\\\\");
    let cmakeuserpresets_content = format!(
        r#"{{
            "version": 3,
            "configurePresets": [
              {{
                "name": "dev-default",
                "displayName": "Developer Default (Debug)",
                "description": "Sets VCPKG_ROOT for local development. DO NOT COMMIT THIS FILE.",
                "inherits": "debug",
                "binaryDir": "${{sourceDir}}/build/dev-default",
                "environment": {{
                  "VCPKG_ROOT": "{}"
                }}
              }}
            ],
            "buildPresets": [
              {{
                "name": "dev-default",
                "configurePreset": "dev-default",
                "displayName": "Build (Dev Default)"
              }}
            ]
          }}"#,
        vcpkg_root_json_escaped
    );
    write_file_content(
        &project_path.join("CMakeUserPresets.json"),
        &cmakeuserpresets_content,
    )?;

    // ... (README.md creation) ...

    // 9. Create .gitignore
    let gitignore_content = r#"# CMake general
          build/
          install/
          CMakeUserPresets.json
          
          # CMake generated files at root (if any)
          CMakeCache.txt
          CMakeFiles/
          cmake_install.cmake # Can also appear at root
          
          # vcpkg
          vcpkg_installed/
          
          # IDE specific
          .vscode/
          .idea/
          *.suo
          *.ntvs*
          *.njsproj
          *.sln.docstates
          
          # Compiled Object files
          *.slo
          *.lo
          *.o
          *.obj
          
          # Precompiled Headers
          *.gch
          *.pch
          
          # Compiled Dynamic libraries
          *.so
          *.dylib
          *.dll
          
          # Fortran module files
          *.mod
          *.smod
          
          # Compiled Static libraries
          *.lai
          *.la
          *.a
          *.lib
          
          # Executables
          *.exe
          *.out
          *.app
          "#;
    write_file_content(&project_path.join(".gitignore"), gitignore_content)?;

    println!("\nProject '{}' created successfully!", project_name);
    println!("  Path: {:?}", fs::canonicalize(&project_path)?);
    println!("\nNext steps:");
    println!("1. `cd {}`", project_name);
    println!("2. Configure: `cmake --preset dev-default`"); // This is your main development preset
    println!("3. Build: `cmake --build --preset dev-default`"); // Use the matching build preset
    println!(
        "4. Run your executable (e.g., `./build/dev-default/{}` or `build\\dev-default\\{}.exe`)",
        project_name, project_name
    ); // Adjusted path
    println!("\nTo build a release version (after configuring it):");
    println!(
        "1. Configure: `cmake --preset release` (VCPKG_ROOT must be in env or CMakeUserPresets modified for release)"
    );
    println!("2. Build: `cmake --build --preset release`");
    println!(
        "\nRemember to add `CMakeUserPresets.json` to your global gitignore or ensure it's in the project's .gitignore (it is by default with this script)."
    );

    Ok(())
}

fn write_file_content(path: &Path, content: &str) -> Result<(), io::Error> {
    let mut file = fs::File::create(path)?;
    file.write_all(content.as_bytes())?;
    println!("Created file: {:?}", path);
    Ok(())
}

fn run_command(
    command: &Path,
    args: &[&str],
    current_dir: Option<&Path>,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd_instance = OsCommand::new(command);
    cmd_instance.args(args);
    if let Some(dir) = current_dir {
        cmd_instance.current_dir(dir);
    }

    println!(
        "Executing: {} {} (in {:?})",
        command.file_name().unwrap_or_default().to_string_lossy(),
        args.join(" "),
        current_dir.unwrap_or_else(|| Path::new("."))
    );

    let status = cmd_instance.status()?;

    if !status.success() {
        return Err(format!(
            "Command failed: {:?} {:?} (exit code: {:?})",
            command,
            args,
            status.code()
        )
        .into());
    }
    Ok(())
}
