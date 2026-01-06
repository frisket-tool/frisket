/*
 * (C) Copyright 2025-2026 Jan Philipp Thiele
 * SPDX-License-Identifier: Apache-2.0 OR MIT
 */
pub mod config;

use std::process::{Command, Output};

pub fn texfile_default_check(filename: String) -> String {
    if filename.is_empty() {
        let conf = config::read_project_config();
        return conf.main;
    } else {
        return filename;
    }
}

fn get_texmfhome() -> String {
    let mut pwd = std::env::current_dir().unwrap();
    pwd.push(".texenv");
    let texmfhome = pwd.to_str().unwrap().to_owned();
    return texmfhome;
}

pub fn exec_within_texenv(program: &str, args: Vec<String>) -> Output {
    let output = Command::new(program)
        .env("TEXMFHOME", get_texmfhome())
        .args(args)
        .output()
        .expect("File not found!");
    return output;
}
