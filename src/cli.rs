// src/cli.rs
use clap::{Parser, Subcommand};

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
        // ... existing fields ...
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
        /// CMake preset to use for building
        #[clap(long, short, default_value = "dev")]
        preset: String,

        /// Perform a clean build (removes existing build directory for the preset)
        #[clap(long)]
        clean: bool,
    },

    /// Runs the project's executable (builds first if necessary)
    Run {
        /// CMake preset to use for building and finding the executable
        #[clap(long, short, default_value = "dev")]
        preset: String,

        /// Name of the executable target. Defaults to the project directory name.
        #[clap(long, short)]
        target: Option<String>,

        /// Perform a clean build before running
        #[clap(long)]
        clean: bool,

        /// Arguments to pass to the executable
        #[clap(last = true)]
        executable_args: Vec<String>,
    },
}