/*
 * (C) Copyright 2025 Jan Philipp Thiele
 * SPDX-License-Identifier: Apache-2.0 OR MIT
 */

fn texengine_handle_package_error(errorline: &str) {
    let errorpackage = errorline.split_whitespace().nth(2).unwrap();
    println!("Error from package {}!", errorpackage);
}

fn texengine_handle_latex_error(errorline: &str) {
    let errortype = errorline.split_whitespace().nth(3).unwrap();
    if errortype.eq("File") {
        let missingfile = errorline.split_whitespace().nth(4).unwrap();
        let missingfile = missingfile.replace(&['\'', '`'], "");
        println!(
            "Missing file {}, searching for it through tlmgr",
            missingfile
        );
        crate::tlmgr::search_file(missingfile.as_str());
    }
}

fn texengine_error_match(errorline: &str) {
    // println!("{}", errorline);
    if errorline.starts_with("! LaTeX Error:") {
        texengine_handle_latex_error(errorline);
    } else if errorline.starts_with("! Package") {
        texengine_handle_package_error(errorline);
    } else if errorline.starts_with("! Emergency stop") {
    } else if errorline.starts_with("!  ==> Fatal") {
    } else {
        println!("Unknown build error!")
    }
}

pub fn texengine_error_check(outputstring: String) {
    for part in outputstring.lines() {
        if part.starts_with("!") {
            texengine_error_match(part);
        }
    }
}

pub unsafe fn run_engine(filename: String) {
    let output = crate::core::exec_within_texenv(
        "pdflatex",
        vec![crate::core::texfile_default_check(filename)],
    );
    if !output.status.success() {
        crate::build::texengine_error_check(String::from_utf8(output.stdout).unwrap());
    }
}
