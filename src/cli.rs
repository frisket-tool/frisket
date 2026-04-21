/*
 * (C) Copyright 2026 Jan Philipp Thiele
 * SPDX-License-Identifier: Apache-2.0 OR MIT
 */

use clap::{Parser, Subcommand, ValueEnum};
use clap_complete::Shell;


#[derive(Parser, Debug)]
#[command(author = "Jan Philipp Thiele", version)]
#[command(name = "frisket")]
#[command(about = "A CLI tool for managing your LaTeX projects")]
pub struct Args {
    #[command(subcommand)]
    pub command: Cmd,
}


#[derive(Subcommand, Debug)]
pub enum Cmd {
    /// Build the main TeX document specified in texproject.toml
    #[command(name = "build")]
    Build {
        /// (optional) Name of different TeX file to build
        #[arg(default_value = "")]
        filename: String,
        /// Toolchain for building
        #[clap(value_enum,long, default_value_t)]
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
        /// Year of TexLive version to use in the project
        #[arg(short,long,default_value="2026")]
        year: i16
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
    /// Install all dependencies as given in the project TOML
    #[command(name = "instantiate")]
    Instantiate {},
    /// Generate shell completions
    #[command(name= "completions")]
    Completions {
        /// Shell to generate completions for
        #[arg(value_enum)]
        shell: Shell
    },
    /// setup directories for frisket
    #[command(name="setup")]
    Setup{ },
    /// Install TinyTeX 
    #[command(name="install")]
    Install{ 
        #[arg(default_value="2026")]
        year: i16
    },
}


#[derive(ValueEnum, Default, Clone, Debug)]
pub enum Toolchain {
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

pub fn toolchain_to_commandstring(toolchain: Toolchain) -> String {
    match toolchain {
        Toolchain::Quick => return String::from("T"),
        Toolchain::Full => return String::from("TBTT"),
        Toolchain::Check => return String::from("FLS"),
        Toolchain::Custom => return String::from("custom"),
        Toolchain::Tex => return String::from("T"),
        Toolchain::Bib => return String::from("B"),
        Toolchain::Lint => return String::from("L"),
        Toolchain::Format => return String::from("F"),
        Toolchain::Spellcheck => return String::from("S"),
    }
}