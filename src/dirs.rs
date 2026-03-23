/*
 * (C) Copyright 2025-2026 Jan Philipp Thiele
 * SPDX-License-Identifier: Apache-2.0 OR MIT
 */

use std::path::{PathBuf};

use xdg::BaseDirectories;

pub fn basedir() -> BaseDirectories{
    return xdg::BaseDirectories::with_prefix("frisket");
}

pub fn data_home() -> PathBuf{
    return basedir().get_data_home().unwrap();
}

pub fn tinytex_repo() -> PathBuf{
    let mut path = data_home();
    path.push("tinytex-releases.git");
    return path;
}