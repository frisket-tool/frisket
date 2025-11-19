/*
 * (C) Copyright 2025 Jan Philipp Thiele
 * SPDX-License-Identifier: Apache-2.0 OR MIT
 */

mod add;
mod build;
mod core;
mod init;
mod remove;
mod tlmgr;

use clap::{Parser, Subcommand};
// use toml_edit::{DocumentMut, value};

#[derive(Parser, Debug)]
#[command(author = "Jan Philipp Thiele", version = "0.1.0")]
struct Args {
    #[command(subcommand)]
    command: Cmd,
}

#[derive(Subcommand, Debug)]
enum Cmd {
    /// Build the main TeX document specified in texproject.toml
    #[command(name = "build")]
    Build {
        /// (optional) Name of different TeX file to build
        #[arg(default_value = "")]
        filename: String,
    },
    /// Initialize new TeX project in directory
    #[command(name = "init")]
    Init {
        /// Name of the directory for the TeX project
        #[arg()]
        directory: String,
    },
    /// Add a package dependency to the current project
    #[command(name = "add")]
    Add {
        #[arg()]
        packagename: String,
    },
    /// Remove a package dependency from the current project
    #[command(name = "remove")]
    Remove {
        #[arg()]
        packagename: String,
    },
}

fn main() {
    let args = Args::parse();
    match args.command {
        Cmd::Add { packagename } => {
            crate::add::package(&packagename);
        }
        Cmd::Remove { packagename } => {
            crate::remove::package(&packagename);
        }
        Cmd::Build { filename } => unsafe { crate::build::run_engine(filename) },
        Cmd::Init { directory } => crate::init::initialize_project_directory(directory),
    }
}
