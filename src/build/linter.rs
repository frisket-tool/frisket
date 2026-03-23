/*
 * (C) Copyright 2026 Jan Philipp Thiele
 * SPDX-License-Identifier: Apache-2.0 OR MIT
 */
use std::process::{Command};

use crate::{core::config::texlive_version, tinytex};

pub fn run(filename:String){
    let conf = crate::core::config::read_project_config();
    let bin = tinytex::binary_string(texlive_version(),&conf.tools.lint.as_str());
    let output = Command::new(bin)
        .arg(crate::core::texfile_default_check(filename.clone()))
        .output()
        .expect("File not found!");
    print!("{}",String::from_utf8(output.stdout).unwrap());
}