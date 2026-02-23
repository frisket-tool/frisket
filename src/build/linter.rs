/*
 * (C) Copyright 2026 Jan Philipp Thiele
 * SPDX-License-Identifier: Apache-2.0 OR MIT
 */
use std::process::{Command};

pub fn run(filename:String){
    let conf = crate::core::config::read_project_config();
    let output = Command::new(&conf.tools.lint.as_str())
        .arg(crate::core::texfile_default_check(filename.clone()))
        .output()
        .expect("File not found!");
    print!("{}",String::from_utf8(output.stdout).unwrap());
}