/*
 * (C) Copyright 2026 Jan Philipp Thiele
 * SPDX-License-Identifier: Apache-2.0 OR MIT
 */

use std::fs::create_dir_all;

use crate::tinytex;

pub fn initial() {
    create_directories();
}

fn create_directories() {
    //XDG DATA/frisket
    let tt_repo_dir = crate::dirs::tinytex_repo();
    create_dir_all(tt_repo_dir).unwrap();
    //tinytex-releases bare repo
    tinytex::clone_bare_releases_repo();
    //latest tinytex release
    tinytex::setup_release_dirs();

    //TODO: template repo
    //XDG config?
}
