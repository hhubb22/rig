// src/cmake.rs
use crate::config::ProjectConfig;
use std::path::Path;

pub(crate) const CMAKELISTS_FILENAME: &str = "CMakeLists.txt";
pub(crate) const CMAKE_PRESETS_FILENAME: &str = "CMakePresets.json";
pub(crate) const CMAKE_USER_PRESETS_FILENAME: &str = "CMakeUserPresets.json";


pub fn generate_cmakelists_content(config: &ProjectConfig) -> String {
    let find_package_lines = config.dependencies
        .iter()
        .filter(|d| !d.is_empty())
        .map(|dep| format!("find_package({} CONFIG REQUIRED)", dep))
        .collect::<Vec<_>>()
        .join("\n");

    let link_libraries_lines = config.dependencies
        .iter()
        .filter(|d| !d.is_empty())
        .map(|dep| format!("{}::{}", dep, dep))
        .collect::<Vec<_>>()
        .join(" ");

    format!(
        r#"cmake_minimum_required(VERSION 3.19)
project({project_name} CXX)

set(CMAKE_CXX_STANDARD {cpp_standard})
set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CMAKE_EXPORT_COMPILE_COMMANDS ON)

# Vcpkg integration
{find_package_lines}

add_executable({project_name} {main_cpp_file})

target_link_libraries({project_name} PRIVATE {link_libraries_lines})
"#,
        project_name = config.project_name,
        main_cpp_file = config.main_cpp_file,
        cpp_standard = config.cpp_standard,
        find_package_lines = if find_package_lines.is_empty() { "// No dependencies specified".to_string() } else { find_package_lines },
        link_libraries_lines = if link_libraries_lines.is_empty() { "// No dependencies to link".to_string() } else { link_libraries_lines }
    )
}

pub fn generate_cmakepresets_content() -> String {
    r#"{
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
            "inherits": "vcpkg-base",
            "cacheVariables": { "CMAKE_BUILD_TYPE": "Debug" }
        },
        {
            "name": "release",
            "displayName": "Release Build",
            "inherits": "vcpkg-base",
            "cacheVariables": { "CMAKE_BUILD_TYPE": "Release" }
        }
    ],
    "buildPresets": [
        { "name": "debug", "configurePreset": "debug" },
        { "name": "release", "configurePreset": "release" }
    ],
    "testPresets": [
      { "name": "debug", "configurePreset": "debug", "output": {"outputOnFailure": true}, "execution": {"noTestsAction": "error", "stopOnFailure": true} },
      { "name": "release", "configurePreset": "release", "output": {"outputOnFailure": true}, "execution": {"noTestsAction": "error", "stopOnFailure": true} }
    ]
}"#.to_string()
}

pub fn generate_cmakeuserpresets_content(vcpkg_root_path: &Path) -> String {
    let vcpkg_root_json_escaped = vcpkg_root_path.to_string_lossy().replace('\\', "\\\\");
    format!(
        r#"{{
    "version": 3,
    "configurePresets": [
        {{
            "name": "dev",
            "displayName": "Developer Default (Debug)",
            "inherits": "debug",
            "environment": {{
                "VCPKG_ROOT": "{}",
                "CMAKE_MAKE_PROGRAM": "{}",
                "CMAKE_C_COMPILER": "{}",
                "CMAKE_CXX_COMPILER": "{}"
            }}
        }}
    ],
    "buildPresets": [ {{ "name": "dev", "configurePreset": "dev" }} ],
    "testPresets": [ {{ "name": "dev", "configurePreset": "dev" }} ]
}}"#,
        vcpkg_root_json_escaped,
        if cfg!(windows) {
            "ninja.exe"
        } else {
            "ninja"
        },
        if cfg!(windows) {
            "cl.exe"
        } else {
            "clang"
        },
        if cfg!(windows) {
            "cl.exe"
        } else {
            "clang++"
        }
    )
}