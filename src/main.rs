use std::io;

/*
 * (C) Copyright 2025-2026 Jan Philipp Thiele
 * SPDX-License-Identifier: Apache-2.0 OR MIT
 */
use clap::{CommandFactory,Parser};
use clap_complete::{generate};

mod add;
mod build;
mod core;
mod init;
mod instantiate;
mod remove;
mod tlmgr;
mod cli;


fn main() {
    let args = cli::Args::parse();
    match args.command {
        cli::Cmd::Add { packagename } => {
            crate::add::package(&packagename);
        }
        cli::Cmd::Remove { packagename } => {
            crate::remove::package(&packagename);
        }
        cli::Cmd::Build {
            filename,
            toolchain,
            auto,
        } => unsafe {
            crate::build::run_toolchain(filename, cli::toolchain_to_commandstring(toolchain), auto)
        },
        cli::Cmd::Init { directory } => crate::init::initialize_project_directory(directory),
        cli::Cmd::Instantiate {} => crate::instantiate::instantiate(),
        cli::Cmd::Completions { shell } => {
            let mut cmd = cli::Args::command();
            generate(shell, &mut cmd, "frisket", &mut io::stdout());
        }
    }
}
