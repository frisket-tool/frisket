/*
 * (C) Copyright 2025 Jan Philipp Thiele
 * SPDX-License-Identifier: Apache-2.0 OR MIT
 */

pub fn instantiate() {
    let conf = crate::core::config::read_project_config();
    for (dep, depsrc) in conf.dependencies {
        if depsrc.to_string().contains("texlive") {
            println!("Installing TeXLive package {}", dep);
            crate::tlmgr::install_pkg(dep.as_str());
        }
    }
}
