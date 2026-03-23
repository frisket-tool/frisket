/*
 * (C) Copyright 2025-2026 Jan Philipp Thiele
 * SPDX-License-Identifier: Apache-2.0 OR MIT
 */

use std::{
    path::Path,
    process::{self, Command},
};

use dialoguer::Confirm;

use crate::{add, core::config::texlive_version, tinytex};

pub fn init_usertree() {
    crate::core::exec_within_texenv("tlmgr", vec!["init-usertree".to_string()]);
}

pub fn install_pkg(pkgname: &str) {
    let args = vec![
        "--usermode".to_string(),
        "install".to_string(),
        pkgname.to_string(),
    ];
    let output = crate::core::exec_within_texenv("tlmgr", args);
    if !output.status.success() {
        println!(
            "Error installing package from TexLive, see following tlmgr output for more details:"
        );
        println!("{}", String::from_utf8(output.stdout).unwrap());
        println!("{}", String::from_utf8(output.stderr).unwrap());
        process::abort();
    }
}

pub fn remove_pkg(pkgname: &str) {
    let args = vec![
        "--usermode".to_string(),
        "remove".to_string(),
        pkgname.to_string(),
    ];
    let output = crate::core::exec_within_texenv("tlmgr", args);
    if !output.status.success() {
        println!(
            "Error removing package from TexLive, see following tlmgr output for more details:"
        );
        println!("{}", String::from_utf8(output.stdout).unwrap());
        println!("{}", String::from_utf8(output.stderr).unwrap());
        process::abort();
    }
}
pub fn search_file_and_install_pkg(filename: &str, automode: bool) {
    let bin = tinytex::binary_string(texlive_version(), "tlmgr");
    let output = Command::new(bin)
        .arg("search")
        .arg("--global")
        .arg("--file")
        .arg(filename)
        .output()
        .expect("tlmgr error searching file!");

    if !output.status.success() {
        println!("Something went wrong searching for file {}", filename);
        println!("{}", String::from_utf8(output.stderr).unwrap());
    }

    let stdoutstring = String::from_utf8(output.stdout).unwrap();
    let mut packages = vec![];
    let mut pkgname = "";
    for line in stdoutstring.lines() {
        if line.starts_with("tlmgr:") {
            continue;
        }
        if line.starts_with("	") {
            let filepath = Path::new(line);
            let foundfile = filepath.file_name().unwrap();
            if foundfile.eq_ignore_ascii_case(filename) {
                packages.push(pkgname);
            }
        } else {
            pkgname = line.trim_end_matches(":");
        }
    }
    if packages.is_empty() {
        println!("File not found through TeXLive manager.\n Exiting.");
        process::abort();
    } else if packages.len() == 1 {
        let mut confirmation = true;
        if !automode {
            confirmation = Confirm::new()
                .with_prompt(format!(
                    "Found {} in {}, do you want to install it?",
                    filename,
                    packages.first().unwrap()
                ))
                .interact()
                .unwrap();
        }
        if confirmation {
            add::package(packages.first().unwrap());
            println!("Package installed, retrying build.");
        } else {
            println!("No package installed, cancelling build.");
            process::abort();
        }
    } else {
    }
}
