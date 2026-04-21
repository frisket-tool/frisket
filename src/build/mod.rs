/*
 * (C) Copyright 2025-2026 Jan Philipp Thiele
 * SPDX-License-Identifier: Apache-2.0 OR MIT
 */

mod linter;
mod texengine;

use std::process::{self, Command};

use crate::{core::config::texlive_version, tinytex::binary_string};

pub unsafe fn run_toolchain(filename: String, toolchain: String, automode: bool) {
    let conf = crate::core::config::read_project_config();
    let mut _toolchain = toolchain;
    if _toolchain.contains("custom") {
        _toolchain = conf.tools.custom_toolchain;
        println!("running custom toolchain: {}", _toolchain);
    } else {
        println!("running toolchain: {}", _toolchain);
    }

    for c in _toolchain.chars() {
        match c {
            'B' => run_bib(filename.clone()),
            'T' => texengine::run(filename.clone(), automode),
            'L' => linter::run(filename.clone()),
            'F' => run_format(filename.clone()),
            'S' => run_spellcheck(),
            _ => {
                println!("unknown toolchain command {}", c);
                println!("options are:");
                println!("- T: texengine");
                println!("- B: bibliography");
                println!("- L: lint");
                println!("- F: format");
                println!("- S: spellcheck");
                process::abort();
            }
        }
    }
}

pub fn run_bib(filename: String) {
    let conf = crate::core::config::read_project_config();
    let output = crate::core::exec_within_texenv(
        &conf.tools.bib.as_str(),
        vec![crate::core::texfile_default_check(filename.clone())],
    );
    if !output.status.success() {
        println!("error in building bibliography!");
        println!("{}", String::from_utf8(output.stdout).unwrap());
        println!("{}", String::from_utf8(output.stderr).unwrap());
        process::abort()
    }
    println!("Bibliography build successfull.")
}

pub fn run_format(filename: String) {
    //Todo: format all tex files in folder?!
    let conf = crate::core::config::read_project_config();
    let output = crate::core::exec_within_texenv(
        &&conf.tools.format.as_str(),
        vec![crate::core::texfile_default_check(filename.clone())],
    );
    print!("{}", String::from_utf8(output.stderr).unwrap());
    if !output.status.success() {
        println!("error during formatting!");
        println!("{}", String::from_utf8(output.stdout).unwrap());
        process::abort()
    }
    println!("Formatting successfull.")
}

pub fn run_spellcheck() {
    let conf = crate::core::config::read_project_config();
    let bin = binary_string(texlive_version(), &conf.tools.spellcheck.as_str());
    let output = Command::new(bin).output().unwrap();
    print!("{}", String::from_utf8(output.stdout).unwrap());
}
