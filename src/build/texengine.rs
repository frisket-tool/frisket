/*
 * (C) Copyright 2026 Jan Philipp Thiele
 * SPDX-License-Identifier: Apache-2.0 OR MIT
 */

use std::process;

use crate::core::exec_within_texenv;

fn handle_package_error(errorline: &str) -> Result<(), String> {
    let errorpackage = errorline.split_whitespace().nth(2).unwrap();
    match errorpackage {
        "pdftex.def" => return handle_package_pdftex_error(errorline),
        _ => return Err(format!("Error from package {errorpackage}!")),
    }
}

fn handle_package_pdftex_error(errorline: &str) -> Result<(), String> {
    let errortype: &str = errorline.split_whitespace().nth(4).unwrap();
    if errortype.eq("File") {
        let filename = errorline
            .split_whitespace()
            .nth(5)
            .unwrap()
            .split_at(1)
            .1
            .trim_end_matches("'");
        if filename.ends_with("eps-converted-to.pdf") {
            let epsfile = [filename.trim_end_matches("-eps-converted-to.pdf"), ".eps"].join("");
            let mut outfilearg = String::from("--outfile=");
            outfilearg.push_str(filename);
            let res = exec_within_texenv("epstopdf", vec![outfilearg, epsfile.to_owned()]);
            if res.status.success() {
                return Ok(());
            } else {
                println!("Error converting with epstopdf");
                print!("{}", String::from_utf8(res.stdout.clone()).unwrap());
                print!("{}", String::from_utf8(res.stderr.clone()).unwrap());
            }
        }
        println!("pdftex.def error: File {} not found", filename);
    }
    //handle ! Package pdftex.def Error: File `<filename>-eps-converted-to.pdf' not found: using draft setting.
    return Err(String::from("Unknown pdftex.def Error"));
}

fn handle_latex_error(errorline: &str, automode: bool) -> Result<(), String> {
    let errortype = errorline.split_whitespace().nth(3).unwrap();
    if errortype.eq("File") {
        let missingfile = errorline.split_whitespace().nth(4).unwrap();
        let missingfile = missingfile.replace(&['\'', '`'], "");
        println!(
            "Missing file {}, searching for it through tlmgr",
            missingfile
        );
        crate::tlmgr::search_file_and_install_pkg(missingfile.as_str(), automode);
        return Ok(());
    }
    return Err(String::from("Unknown LaTeX Error"));
}

fn match_error(errorline: &str, automode: bool) -> Result<(), String> {
    // println!("{}", errorline);
    if errorline.starts_with("! LaTeX Error:") {
        return handle_latex_error(errorline, automode);
    } else if errorline.starts_with("! Package") {
        return handle_package_error(errorline);
    } else if errorline.starts_with("! Emergency stop") {
        return Err(String::from("Emergency stop!"));
    } else if errorline.starts_with("!  ==> Fatal") {
        return Err(String::from("Fatal error"));
    } else {
        return Err(String::from("Unknown build error!"));
    }
}

fn check_error(outputstring: String, automode: bool) -> Result<(), String> {
    for part in outputstring.lines() {
        // println!("{}", part.to_string());
        if part.starts_with("!") {
            return match_error(part, automode);
        }
    }
    return Ok(());
}

pub fn run(filename: String, automode: bool) {
    let conf = crate::core::config::read_project_config();
    let mut output = crate::core::exec_within_texenv(
        conf.tools.tex.as_str(),
        vec![
            // String::from("--shell-escape"),
            crate::core::texfile_default_check(filename.clone()),
        ],
    );
    while !output.status.success() {
        let res = check_error(String::from_utf8(output.stdout.clone()).unwrap(), automode);
        if res.is_err() {
            println!("{}", res.unwrap_err());
            println!("Showing full LaTeX output:");
            // print!("{}", String::from_utf8(output.stdout.clone()).unwrap());
            process::abort();
        } else {
            output = crate::core::exec_within_texenv(
                conf.tools.tex.as_str(),
                vec![crate::core::texfile_default_check(filename.clone())],
            );
        }
    }
    println!("Tex build successful.");
}
