/*
 * (C) Copyright 2025-2026 Jan Philipp Thiele
 * SPDX-License-Identifier: Apache-2.0 OR MIT
 */

use std::path::{PathBuf};

use directories::BaseDirs;

pub fn data_home() -> PathBuf{
    let bd = BaseDirs::new().unwrap();
    let mut basedir = PathBuf::from(bd.data_dir());
    basedir.push("frisket");
    return basedir;
}

pub fn tinytex_repo() -> PathBuf{
    let mut path = data_home();
    path.push("tinytex-releases.git");
    return path;
}