/*
 * (C) Copyright 2025-2026 Jan Philipp Thiele
 * SPDX-License-Identifier: Apache-2.0 OR MIT
 */

use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use toml::Table;

fn default_textools() -> TexTools {
    TexTools {
        tex: "pdflatex".to_string(),
        bib: "bibtext".to_string(),
        lint: "chktext".to_string(),
        format: "tex-fmt".to_string(),
        spellcheck: "codespell".to_string(),
        custom_toolchain: "T".to_string(),
    }
}

pub fn read_project_config() -> TexProject {
    let tomlstring =
        std::fs::read_to_string(PathBuf::from("texproject.toml")).expect("File not found");
    let conf: TexProject = toml::from_str(tomlstring.as_str()).expect("parsing TOML failed");
    return conf;
}

fn write_project_config(toml: TexProject) {
    let tomlstring = toml::to_string_pretty(&toml).expect("serializing TOML failed");
    std::fs::write("texproject.toml", tomlstring).expect("Writing TOML failed");
}

pub fn add_project_dependency(pkg: String, src: String) {
    let mut conf = read_project_config();
    conf.dependencies.insert(pkg, toml::Value::String(src));
    write_project_config(conf);
}

pub fn remove_project_dependency(pkg: &String) {
    let mut conf = read_project_config();
    conf.dependencies.remove(pkg);
    write_project_config(conf);
}

#[derive(Deserialize, Serialize)]
pub struct TexTools {
    pub tex: String,
    pub bib: String,
    pub lint: String,
    pub format: String,
    pub spellcheck: String,
    pub custom_toolchain: String,
}

#[derive(Deserialize, Serialize)]
pub struct TexProject {
    pub main: String,
    #[serde(default = "default_textools")]
    pub tools: TexTools,
    pub dependencies: Table,
}
