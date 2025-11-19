/*
 * (C) Copyright 2025 Jan Philipp Thiele
 * SPDX-License-Identifier: Apache-2.0 OR MIT
 */

use std::path::PathBuf;
use std::process::{Command, Output};
use toml_edit::DocumentMut;

pub fn get_config_toml() -> DocumentMut {
    let toml = std::fs::read_to_string(PathBuf::from("texproject.toml")).expect("File not found");
    let conf = toml.parse::<DocumentMut>().expect("invalid doc");
    return conf;
}

pub fn texfile_default_check(filename: String) -> String {
    if filename.is_empty() {
        let conf = get_config_toml();
        return String::from(conf["project"]["main"].as_str().unwrap());
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
