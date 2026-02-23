/*
 * (C) Copyright 2025-2026 Jan Philipp Thiele
 * SPDX-License-Identifier: Apache-2.0 OR MIT
 */

fn texengine_handle_package_error(errorline: &str) {
    let errorpackage = errorline.split_whitespace().nth(2).unwrap();
    println!("Error from package {}!", errorpackage);
}

fn texengine_handle_latex_error(errorline: &str, automode: bool) {
    let errortype = errorline.split_whitespace().nth(3).unwrap();
    if errortype.eq("File") {
        let missingfile = errorline.split_whitespace().nth(4).unwrap();
        let missingfile = missingfile.replace(&['\'', '`'], "");
        println!(
            "Missing file {}, searching for it through tlmgr",
            missingfile
        );
        crate::tlmgr::search_file_and_install_pkg(missingfile.as_str(), automode);
    }
}

fn texengine_error_match(errorline: &str, automode: bool) {
    // println!("{}", errorline);
    if errorline.starts_with("! LaTeX Error:") {
        texengine_handle_latex_error(errorline, automode);
    } else if errorline.starts_with("! Package") {
        texengine_handle_package_error(errorline);
    } else if errorline.starts_with("! Emergency stop") {
    } else if errorline.starts_with("!  ==> Fatal") {
    } else {
        println!("Unknown build error!")
    }
}

pub fn texengine_error_check(outputstring: String, automode: bool) {
    for part in outputstring.lines() {
        println!("{}", part.to_string());
        if part.starts_with("!") {
            texengine_error_match(part, automode);
            return;
        }
    }
}

pub unsafe fn run_engine(filename: String, automode: bool) {
    let conf = crate::core::config::read_project_config();
    let mut output = crate::core::exec_within_texenv(
        conf.tools.tex.as_str(),
        vec![crate::core::texfile_default_check(filename.clone())],
    );
    while !output.status.success() {
        crate::build::texengine_error_check(String::from_utf8(output.stdout).unwrap(), automode);
        output = crate::core::exec_within_texenv(
            conf.tools.tex.as_str(),
            vec![crate::core::texfile_default_check(filename.clone())],
        );
    }
    println!("Build successfull.");
}
