use std::io;

/*
 * (C) Copyright 2025-2026 Jan Philipp Thiele
 * SPDX-License-Identifier: Apache-2.0 OR MIT
 */
use crate::cli::Cmd::*;
use clap::{CommandFactory, Parser};
use clap_complete::generate;

mod add;
mod build;
mod cli;
mod core;
mod dirs;
mod init;
mod instantiate;
mod remove;
mod setup;
mod tinytex;
mod tlmgr;

fn main() {
    let args = cli::Args::parse();
    match args.command {
        Add { packagename } => {
            crate::add::package(&packagename);
        }
        Remove { packagename } => {
            crate::remove::package(&packagename);
        }
        Build {
            filename,
            toolchain,
            auto,
        } => unsafe {
            crate::build::run_toolchain(filename, cli::toolchain_to_commandstring(toolchain), auto)
        },
        Init { directory, year } => crate::init::initialize_project_directory(directory,year),
        Instantiate {} => crate::instantiate::instantiate(),
        Completions { shell } => {
            let mut cmd = cli::Args::command();
            generate(shell, &mut cmd, "frisket", &mut io::stdout());
        }
        Setup {} => crate::setup::initial(),
        Install { year } => tinytex::install(year),
    }
}
