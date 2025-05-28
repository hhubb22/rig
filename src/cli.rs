// src/cli.rs
use clap::{Parser, Subcommand, Args as ClapArgs};

#[derive(Parser)]
#[clap(author, version, about = "A CLI tool to create C++/CMake/vcpkg projects", long_about = None)]
pub struct Args {
    #[command(subcommand)]
    pub command: CliCommand,
}

#[derive(Subcommand)]
pub enum CliCommand {
    /// Creates a new C++ project with CMake and vcpkg
    New {
        name: String,
        #[clap(long)]
        vcpkg_root: Option<String>,
        #[clap(long, value_delimiter = ',', default_value = "fmt")]
        deps: Vec<String>,
        #[clap(long, default_value = "17")]
        std: String,
    },

    /// Builds the project using a CMake preset
    Build {
        #[clap(long, short, default_value = "dev")]
        preset: String,
        #[clap(long)]
        clean: bool,
    },

    /// Runs the project's executable (builds first if necessary)
    Run {
        #[clap(long, short, default_value = "dev")]
        preset: String,
        #[clap(long, short)]
        target: Option<String>,
        #[clap(long)]
        clean: bool,
        #[clap(last = true)]
        executable_args: Vec<String>,
    },

    /// Adds one or more dependencies to the project using vcpkg
    Add {
        /// Names of the vcpkg ports to add
        #[clap(required = true, num_args = 1..)]
        dependencies: Vec<String>,

        /// Path to the VCPKG_ROOT directory (overrides environment variable)
        #[clap(long)]
        vcpkg_root: Option<String>,
    },

    /// Cleans build artifacts for specified presets or all presets
    Clean(CleanArgs), // Added Clean subcommand
}

#[derive(ClapArgs, Debug)] // Added derive Debug
#[clap(group(
    clap::ArgGroup::new("scope")
        .required(false) // If neither is provided, default to cleaning current dir's concept of active build if possible, or error. For now, one is effectively required.
        .args(&["preset", "all"]),
))]
pub struct CleanArgs {
    /// CMake preset whose build directory to clean
    #[clap(long, short)]
    pub preset: Option<String>,

    /// Clean all build directories for all presets
    #[clap(long, action = clap::ArgAction::SetTrue)] // Ensures --all acts as a flag
    pub all: bool,
    // Note: clap will ensure 'preset' and 'all' are not used together if not desired,
    // or we can add custom validation logic in the handler.
    // For now, we'll let the action logic handle the precedence (e.g., 'all' overrides 'preset').
}