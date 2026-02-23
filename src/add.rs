/*
 * (C) Copyright 2025-2026 Jan Philipp Thiele
 * SPDX-License-Identifier: Apache-2.0 OR MIT
 */

pub fn package(pkgname: &str) {
    println!("Adding package: {}", pkgname);
    crate::tlmgr::install_pkg(pkgname);
    crate::core::config::add_project_dependency(pkgname.to_string(), "texlive".to_string());
}
