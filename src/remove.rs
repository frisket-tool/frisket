/*
 * (C) Copyright 2025 Jan Philipp Thiele
 * SPDX-License-Identifier: Apache-2.0 OR MIT
 */

pub fn package(pkgname: &str) {
    let conf = crate::core::config::read_project_config();
    if !conf.dependencies.contains_key(pkgname) {
        println!("Error: dependency not found in configuration!");
        std::process::abort()
    }
    let pkgsrc = conf.dependencies.get(pkgname).expect("foo").to_string();
    if pkgsrc.contains("texlive") {
        println!("Removing TexLive package {}", pkgname);
        crate::tlmgr::remove_pkg(pkgname);
    }
    crate::core::config::remove_project_dependency(&pkgname.to_string());
}
