// src/main.rs
mod cli;
mod cmake;
mod config;
mod utils;
mod vcpkg;
mod actions;

use anyhow::Result;
use clap::Parser;

use cli::{Args, CliCommand, CleanArgs}; // Added CleanArgs

fn main() -> Result<()> {
    let args = Args::parse();

    match args.command {
        CliCommand::New {
            name,
            vcpkg_root,
            deps,
            std,
        } => {
            actions::new::new_project(name, vcpkg_root, deps, std)?;
        }
        CliCommand::Build { preset, clean } => {
            actions::build::build_project(&preset, clean)?;
        }
        CliCommand::Run {
            preset,
            target,
            clean,
            executable_args,
        } => {
            actions::run::run_project(&preset, target, clean, &executable_args)?;
        }
        CliCommand::Add {
            dependencies,
            vcpkg_root,
        } => {
            actions::add::add_dependencies(&dependencies, vcpkg_root)?;
        }
        CliCommand::Clean(CleanArgs { preset, all }) => { // Added handler for Clean
            actions::clean::clean_project(preset, all)?;
        }
    }
    Ok(())
}