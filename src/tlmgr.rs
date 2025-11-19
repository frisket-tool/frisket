/*
 * (C) Copyright 2025 Jan Philipp Thiele
 * SPDX-License-Identifier: Apache-2.0 OR MIT
 */

use std::{path::Path, process::Command};

use dialoguer::Confirm;

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
    // if !tlmgroutput.status.success() {
    println!("{}", String::from_utf8(output.stdout).unwrap());
    println!("{}", String::from_utf8(output.stderr).unwrap());
    // }
}

pub fn remove_pkg(pkgname: &str) {
    let args = vec![
        "--usermode".to_string(),
        "remove".to_string(),
        pkgname.to_string(),
    ];
    let output = crate::core::exec_within_texenv("tlmgr", args);
    // if !tlmgroutput.status.success() {
    println!("{}", String::from_utf8(output.stdout).unwrap());
    println!("{}", String::from_utf8(output.stderr).unwrap());
    // }
}
pub fn search_file(filename: &str) {
    let output = Command::new("tlmgr")
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
    } else if packages.len() == 1 {
        let confirmation = Confirm::new()
            .with_prompt(format!(
                "Found {} in {}, do you want to install it?",
                filename,
                packages.first().unwrap()
            ))
            .interact()
            .unwrap();

        if confirmation {
            install_pkg(packages.first().unwrap());
            println!("Retrying build! (TODO)");
        }
    } else {
    }
}
