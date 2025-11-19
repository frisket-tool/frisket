/*
 * (C) Copyright 2025 Jan Philipp Thiele
 * SPDX-License-Identifier: Apache-2.0 OR MIT
 */

pub fn package(pkgname: &str) {
    println!("Adding package: {}", pkgname);
    crate::tlmgr::install_pkg(pkgname);
    //Todo: add to TOML
}
