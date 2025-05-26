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
}