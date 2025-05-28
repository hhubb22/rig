# Rig 🦀: Streamlined C++/CMake/vcpkg Project Management

Rig is a command-line interface (CLI) tool designed to simplify the creation and management of C++ projects that use CMake for building and vcpkg for dependency management. It automates boilerplate setup, provides convenient commands for building and running, and helps manage vcpkg dependencies.

## Table of Contents

- [Rig 🦀: Streamlined C++/CMake/vcpkg Project Management](#rig--streamlined-ccmakevcpkg-project-management)
  - [Table of Contents](#table-of-contents)
  - [Why Rig?](#why-rig)
  - [Features](#features)
  - [Prerequisites](#prerequisites)
  - [Installation](#installation)
    - [From Source](#from-source)
    - [From Crates.io (Future)](#from-cratesio-future)
  - [Core Concepts](#core-concepts)
    - [CMake Presets](#cmake-presets)
    - [vcpkg](#vcpkg)
  - [Usage](#usage)
    - [Quick Start](#quick-start)
    - [Commands](#commands)
      - [`rig new`](#rig-new)
      - [`rig build`](#rig-build)
      - [`rig run`](#rig-run)
      - [`rig add`](#rig-add)
  - [Environment Variables](#environment-variables)
  - [Generated Project Structure](#generated-project-structure)
  - [Contributing](#contributing)
  - [License](#license)
  - [Future Ideas](#future-ideas)

## Why Rig?

Setting up a modern C++ project with CMake and vcpkg involves several manual steps and configuration files. Rig aims to:

*   **Reduce Boilerplate:** Automatically generate `CMakeLists.txt`, `CMakePresets.json`, `vcpkg.json`, and a basic `main.cc`.
*   **Enforce Conventions:** Promote a consistent project structure and build setup using CMake Presets.
*   **Streamline Workflow:** Provide simple commands for common tasks like configuring, building, running, and adding dependencies.
*   **Integrate vcpkg:** Make vcpkg integration seamless from project inception.

## Features

*   **Project Scaffolding:** Create new C++/CMake/vcpkg projects with a single command.
*   **CMake Preset Integration:** Generates and utilizes `CMakePresets.json` and `CMakeUserPresets.json` for reproducible builds.
*   **vcpkg Manifest Management:** Initializes `vcpkg.json` and allows adding dependencies easily.
*   **Simplified Build Process:** Wraps CMake build commands with preset support.
*   **Easy Execution:** Run project executables built with specific presets.
*   **Dependency Addition:** Add new vcpkg packages to your project.

## Prerequisites

Before using Rig, you need the following installed on your system:

1.  **Rust and Cargo:** To build Rig itself. (Visit [rustup.rs](https://rustup.rs/))
2.  **A C++ Compiler:** (e.g., GCC, Clang, MSVC)
3.  **CMake:** Version 3.19 or higher (due to `CMakePresets.json` usage).
4.  **vcpkg:**
    *   Clone the [vcpkg repository](https://github.com/microsoft/vcpkg).
    *   Bootstrap vcpkg (e.g., `./bootstrap-vcpkg.sh` or `.\bootstrap-vcpkg.bat`).
    *   **Crucially, set the `VCPKG_ROOT` environment variable** to point to your vcpkg installation directory. Rig relies on this to find the vcpkg executable and toolchain file.
        ```bash
        # Example for Linux/macOS (add to .bashrc, .zshrc, etc.)
        export VCPKG_ROOT=/path/to/your/vcpkg
        # Example for Windows (PowerShell)
        $env:VCPKG_ROOT = "C:\path\to\your\vcpkg"
        # Or set it system-wide
        ```

## Installation

### From Source

1.  Clone this repository:
    ```bash
    git clone https://your-repo-url/rig.git # Replace with actual URL
    cd rig
    ```
2.  Build and install the binary using Cargo:
    ```bash
    cargo install --path .
    ```
    This will install the `rig` executable into your Cargo bin directory (usually `~/.cargo/bin/`). Ensure this directory is in your system's `PATH`.

    Alternatively, for a release build:
    ```bash
    cargo build --release
    # The executable will be at target/release/rig
    # You can then copy it to a directory in your PATH.
    ```

### From Crates.io (Future)

Once published, you'll be able to install Rig using:

```bash
cargo install rig
```

## Core Concepts

### CMake Presets

Rig heavily utilizes [CMake Presets](https://cmake.org/cmake/help/latest/manual/cmake-presets.7.html) (`CMakePresets.json` and `CMakeUserPresets.json`). This allows defining common configure, build, and test settings that can be easily shared and selected. Rig generates default presets:

*   `vcpkg-base` (hidden): Base preset for vcpkg integration.
*   `debug`: Inherits from `vcpkg-base`, sets `CMAKE_BUILD_TYPE` to `Debug`.
*   `release`: Inherits from `vcpkg-base`, sets `CMAKE_BUILD_TYPE` to `Release`.
*   `dev`: A user preset (in `CMakeUserPresets.json`) that inherits from `debug` and sets the `VCPKG_ROOT` environment variable for CMake. This is typically the default preset for development.

### vcpkg

Rig uses vcpkg in [manifest mode](https://learn.microsoft.com/en-us/vcpkg/users/manifests) via a `vcpkg.json` file. This file declares your project's dependencies, which vcpkg then manages.

## Usage

### Quick Start

1.  **Create a new project:**
    ```bash
    rig new my_awesome_project --deps fmt,spdlog
    ```
2.  **Navigate into the project directory:**
    ```bash
    cd my_awesome_project
    ```
3.  **Configure and Build (using the default 'dev' preset):**
    ```bash
    # Configure (only needed once per preset, or if CMakeLists.txt changes)
    cmake --preset dev 
    # Build
    cmake --build --preset dev
    # Or, using rig's wrapper:
    rig build --preset dev 
    ```
4.  **Run your application:**
    ```bash
    ./build/dev/my_awesome_project
    # Or, using rig's wrapper:
    rig run --preset dev
    ```

### Commands

#### `rig new`

Creates a new C++ project with CMake and vcpkg.

```bash
rig new <name> [OPTIONS]
```

**Arguments:**

*   `<name>`: The name of the project (and the directory that will be created).

**Options:**

*   `--vcpkg-root <VCPKG_ROOT>`:
    Path to the VCPKG_ROOT directory. Overrides the `VCPKG_ROOT` environment variable for this command.
*   `--deps <DEPS>`:
    Comma-separated list of initial vcpkg dependencies to add (e.g., `fmt,spdlog,nlohmann-json`). Default: `fmt`.
*   `--std <STD>`:
    C++ standard to set in `CMakeLists.txt` (e.g., `17`, `20`, `23`). Default: `17`.

**Example:**

```bash
rig new my_game --deps sdl2,glm --std 20
```

This will create a directory `my_game/` with the necessary project files.

#### `rig build`

Builds the project using a specified CMake preset. It handles running the CMake configure step if necessary, then the build step.

```bash
rig build [OPTIONS]
```

**Options:**

*   `-p, --preset <PRESET>`:
    CMake preset to use for building. Default: `dev`.
*   `--clean`:
    Perform a clean build (removes the existing build directory for the specified preset before configuring and building).

**Example:**

```bash
# Build with the 'dev' preset (default)
rig build

# Clean build with the 'release' preset
rig build --preset release --clean
```

#### `rig run`

Builds the project (if necessary) and then runs the executable.

```bash
rig run [OPTIONS] [-- <EXECUTABLE_ARGS>...]
```

**Options:**

*   `-p, --preset <PRESET>`:
    CMake preset to use for building and finding the executable. Default: `dev`.
*   `-t, --target <TARGET>`:
    Name of the executable target. Defaults to the project directory name (or the `name` field from `vcpkg.json`).
*   `--clean`:
    Perform a clean build before running.
*   `-- <EXECUTABLE_ARGS>...`:
    Arguments to pass to the executable. Any arguments after `--` are passed directly.

**Example:**

```bash
# Run with 'dev' preset, default target
rig run

# Run with 'release' preset, passing arguments to the executable
rig run --preset release -- --input data.txt --verbose
```

#### `rig add`

Adds one or more dependencies to the project using vcpkg. This command will modify your `vcpkg.json` file.

```bash
rig add <DEPENDENCIES>... [OPTIONS]
```

**Arguments:**

*   `<DEPENDENCIES>...`:
    One or more names of the vcpkg ports (libraries) to add.

**Options:**

*   `--vcpkg-root <VCPKG_ROOT>`:
    Path to the VCPKG_ROOT directory. Overrides the `VCPKG_ROOT` environment variable for this command.

**Example:**

```bash
# Add a single dependency
rig add nlohmann-json

# Add multiple dependencies
rig add eigen3 range-v3
```

**Important:** After adding dependencies with `rig add`, you will likely need to:
1.  Update your `CMakeLists.txt` to include `find_package(<dependency_name> CONFIG REQUIRED)`.
2.  Link against the imported targets (e.g., `target_link_libraries(${PROJECT_NAME} PRIVATE <dependency_name>::<dependency_name>)`).
3.  Re-run CMake configuration (e.g., `cmake --preset dev` or `rig build` which handles it).

#### `rig clean`

Cleans build artifacts. You can specify a preset to clean its build directory or clean all build directories.

```bash
rig clean [OPTIONS]
```

**Options:**

*   `-p, --preset <PRESET>`:
    CMake preset whose build directory should be cleaned (e.g., `dev`, `release`).
*   `--all`:
    Clean all build directories under the main `build/` folder. This will remove `build/<preset1>`, `build/<preset2>`, etc.

**Mutually Exclusive:**

You should specify either a preset or --all, but not both. If neither is specified, the command will prompt for one. (Note: The current implementation requires one, this can be refined.)

**Example:**

```bash
# Clean build artifacts for the 'dev' preset
rig clean --preset dev

# Clean build artifacts for the 'release' preset
rig clean -p release

# Clean all build artifacts for all presets
rig clean --all
```

**Important:** 

This command removes files and directories. Use with caution, especially the --all flag.

## Environment Variables

*   `VCPKG_ROOT`: Rig relies heavily on this variable to locate your vcpkg installation. Ensure it's set correctly. It can be overridden on a per-command basis using the `--vcpkg-root` option where available.

## Generated Project Structure

When you run `rig new <project_name>`, the following structure is created:

```
<project_name>/
├── .gitignore
├── CMakeLists.txt
├── CMakePresets.json
├── CMakeUserPresets.json
├── main.cc
└── vcpkg.json
```

*   `.gitignore`: Standard ignore file for C++/CMake projects.
*   `CMakeLists.txt`: Main CMake script for building your project.
*   `CMakePresets.json`: Defines standard build presets (e.g., debug, release).
*   `CMakeUserPresets.json`: Defines user-specific presets (e.g., `dev` which sets `VCPKG_ROOT`). This file is typically not committed to version control.
*   `main.cc`: A basic "Hello World" C++ source file.
*   `vcpkg.json`: The vcpkg manifest file declaring project dependencies.

Build artifacts are placed in `build/<preset_name>/`. For example, `build/dev/`.

## Contributing

Contributions are welcome! If you have ideas for improvements or find bugs, please open an issue or submit a pull request.

For development:
1.  Ensure your code is formatted with `cargo fmt`.
2.  Check for linter warnings with `cargo clippy`.
3.  Add tests if you introduce new functionality.

## License

This project is licensed under the MIT License. See the `Cargo.toml` file (or a separate `LICENSE` file if added) for details.

## Future Ideas

*   `rig init`: Initialize Rig structure in an existing C++ project.
*   `rig test`: Support for running CTest with presets.
*   `rig clean <preset|--all>`: More granular cleaning options.
*   `rig update`: Update vcpkg baseline or installed packages.
*   Interactive mode for `rig new` to select options.
*   More sophisticated `CMakeLists.txt` generation (e.g., library projects, tests).
*   Automatic `CMakeLists.txt` updates when adding dependencies (challenging but potentially powerful).

---

Happy C++ Hacking with Rig! 🚀