/*
 * (C) Copyright 2025-2026 Jan Philipp Thiele
 * SPDX-License-Identifier: Apache-2.0 OR MIT
 */

use std::fs::File;
use std::io::Write;

pub fn initialize_project_directory(directory: String) {
    let starting_directory = std::env::current_dir().unwrap();
    let dir = directory.as_str();
    std::fs::create_dir(dir).unwrap();
    // Change into directory
    std::env::set_current_dir(dir).unwrap();
    std::fs::create_dir(".texenv").unwrap();
    // preset??

    // Create texproject.toml
    let mut projecttoml = File::create("texproject.toml").expect("file could not be created");
    indoc::writedoc! {projecttoml,r#"
        main = "main"
        texlive_version = 2026
        [tools]
        tex = "pdflatex"
        bib = "bibtex"
        format = "texfmt"
        lint = "chktex"
        spellcheck = "codespell"
        custom_toolchain = "T"

        [dependencies]
    "#}
    .expect("foo");

    // Create main tex
    let mut mainfile = File::create("main.tex").expect("main file could not be created!");
    indoc::writedoc! {mainfile,r"
        \documentclass[12pt,a4paper]{{article}}
        \title{{My test}}
        \author{{me}}
        \date{{\today}}

        \begin{{document}}
        Foo bar
        \end{{document}}
    "}
    .expect("bar");

    // init tlmgr usertree
    crate::tlmgr::init_usertree();

    std::env::set_current_dir(starting_directory).unwrap();
}
