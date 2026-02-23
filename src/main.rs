/*
 * (C) Copyright 2025-2026 Jan Philipp Thiele
 * SPDX-License-Identifier: Apache-2.0 OR MIT
 */

mod add;
mod build;
mod core;
mod init;
mod instantiate;
mod remove;
mod tlmgr;

use clap::{Parser, Subcommand, ValueEnum};

#[derive(Parser, Debug)]
#[command(author = "Jan Philipp Thiele", version)]
struct Args {
    #[command(subcommand)]
    command: Cmd,
}

#[derive(ValueEnum, Default, Clone, Debug)]
enum Toolchain {
    #[default]
    Quick,
    Full,
    Check,
    Custom,
    Tex,
    Bib,
    Lint,
    Format,
    Spellcheck,
}

fn toolchain_to_commandstring(toolchain: Toolchain) -> String {
    match toolchain {
        Toolchain::Quick => return String::from("T"),
        Toolchain::Full => return String::from("TTBT"),
        Toolchain::Check => return String::from("FLS"),
        Toolchain::Custom => return String::from("custom"),
        Toolchain::Tex => return String::from("T"),
        Toolchain::Bib => return String::from("B"),
        Toolchain::Lint => return String::from("L"),
        Toolchain::Format => return String::from("F"),
        Toolchain::Spellcheck => return String::from("S"),
    }
}
#[derive(Subcommand, Debug)]
enum Cmd {
    /// Build the main TeX document specified in texproject.toml
    #[command(name = "build")]
    Build {
        /// (optional) Name of different TeX file to build
        #[arg(default_value = "")]
        filename: String,
        /// Toolchain for building
        #[clap(value_enum, default_value_t)]
        toolchain: Toolchain,
        /// Auto-Mode: install missing packages without asking
        #[arg(short, long, action)]
        auto: bool,
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
    #[command(name = "instantiate")]
    Instantiate {},
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
        Cmd::Build {
            filename,
            toolchain,
            auto,
        } => unsafe {
            crate::build::run_toolchain(filename, toolchain_to_commandstring(toolchain), auto)
        },
        Cmd::Init { directory } => crate::init::initialize_project_directory(directory),
        Cmd::Instantiate {} => crate::instantiate::instantiate(),
    }
}
