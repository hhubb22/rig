// src/main.rs

// Declare modules
mod cli;
mod cmake;
mod config;
mod project;
mod utils;
mod vcpkg;
mod actions;

use anyhow::Result;
use clap::Parser;

use cli::{Args, CliCommand};
use config::ProjectConfig;

fn main() -> Result<()> {
    let args = Args::parse();

    match args.command {
        CliCommand::New {
            name,
            vcpkg_root,
            deps,
            std,
        } => {
            let config = ProjectConfig::new(name, vcpkg_root, deps, std)?;
            project::create_new_project(&config)?;
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
    }
    Ok(())
}