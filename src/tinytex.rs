/*
 * (C) Copyright 2025-2026 Jan Philipp Thiele
 * SPDX-License-Identifier: Apache-2.0 OR MIT
 */

use git2::build::RepoBuilder;
use std::fs::{File, create_dir_all};
use std::io::Write;
#[cfg(all(target_os = "linux", target_arch = "x86_64"))]
use std::path::PathBuf;
use tempfile::{Builder, TempDir};

//TODO: add option for year to get latest of said year
pub fn get_latest_version_number(year: Option<i16>) -> String {
    let yyyy = year.unwrap_or(2).to_string().to_owned();
    let pattern = ["v", yyyy.as_str(), "*"].join("");
    let repo = git2::Repository::open_bare(crate::dirs::tinytex_repo().as_path()).unwrap();
    let names = repo.tag_names(Some(pattern.as_str())).unwrap();
    let latest = names.iter().rev().nth(0).unwrap();
    return latest.unwrap().to_string();
}

pub fn install(year: i16) {
    let version = get_latest_version_number(Some(year));
    let url = release_url(version.clone());
    let tmp_dir = Builder::new().prefix("frisket-tinytex").tempdir().unwrap();
    println!("Downloading TinyTex {}", version);
    let release_fname = download(url, &tmp_dir);
    unpack_and_move(release_fname, year);
}

fn download(url: String, tmp_dir: &TempDir) -> PathBuf {
    let response = reqwest::blocking::get(url).unwrap();
    let fname = response
        .url()
        .path_segments()
        .and_then(|segments| segments.last())
        .and_then(|name| if name.is_empty() { None } else { Some(name) })
        .unwrap_or("tmp.tar.gz");
    let fname = tmp_dir.path().join(fname);
    let mut dest = { File::create(fname.clone()).unwrap() };
    let content = response.bytes().unwrap();
    dest.write_all(&content).unwrap();
    return fname;
}

#[cfg(target_os = "linux")]
fn unpack_prefix() -> String{
    return ".TinyTeX".to_string();
}
#[cfg(target_os = "macos")]
fn unpack_prefix() -> String{
    return "TinyTeX".to_string();
}

#[cfg(any(target_os = "linux",target_os="macos"))]
fn unpack_and_move(fname: PathBuf, year: i16) {
    use flate2::read::GzDecoder;
    use tar::Archive;
    let tar_gz = File::open(fname.as_path()).unwrap();
    let tar = GzDecoder::new(tar_gz);
    let mut archive = Archive::new(tar);
    let mut tt_dir = crate::dirs::data_home();
    tt_dir.push("TinyTex");
    tt_dir.push(year.to_string());
    println!("Installing into {}", tt_dir.clone().to_str().unwrap());
    let prefix = unpack_prefix();
    archive
        .entries()
        .unwrap()
        .filter_map(|e| e.ok())
        .map(|mut entry| -> Result<PathBuf, Box<dyn std::error::Error>> {
            let path = entry.path()?.strip_prefix(prefix.as_str())?.to_owned();
            let mut dir = tt_dir.clone();
            dir.push(path);
            entry.unpack(&dir)?;
            Ok(dir)
        })
        .filter_map(|e| e.ok())
        .for_each(|x| println!("> {}", x.display()));
}


#[cfg(all(target_os = "linux",target_arch="x86_64"))]
fn release_url(version: String) -> String {
    let result = [
        "https://github.com/rstudio/tinytex-releases/releases/download/",
        version.as_str(),
        "/TinyTex-1-",
        version.as_str(),
        ".tar.gz",
    ]
    .join("");
    return result;
}

#[cfg(all(target_os = "linux",target_arch="aarch64"))]
fn release_url(version: String) -> String {
    let result = [
        "https://github.com/rstudio/tinytex-releases/releases/download/",
        version.as_str(),
        "/TinyTex-1-arm64-",
        version.as_str(),
        ".tar.gz",
    ]
    .join("");
    return result;
}


#[cfg(target_os = "macos")]
fn release_url(version: String) -> String {
    let result = [
        "https://github.com/rstudio/tinytex-releases/releases/download/",
        version.as_str(),
        "/TinyTex-1-",
        version.as_str(),
        ".tgz",
    ]
    .join("");
    return result;
}

fn base_dir(year: i16) -> PathBuf {
    let mut result = crate::dirs::data_home();
    result.push("TinyTeX");
    result.push(year.to_string());
    return result;
}

#[cfg(all(target_os = "linux", target_arch = "x86_64"))]
pub fn binary_dir(year: i16) -> PathBuf {
    let mut bindir = base_dir(year);
    bindir.push("bin");
    bindir.push("x86_64-linux");
    return bindir;
}

#[cfg(all(target_os = "linux", target_arch = "aarch64"))]
pub fn binary_dir(year: i16) -> PathBuf {
    let mut bindir = base_dir(year);
    bindir.push("bin");
    bindir.push("aarch64-linux");
    return bindir;
}

#[cfg(target_os = "macos")]
pub fn binary_dir(year: i16) -> PathBuf {
    let mut bindir = base_dir(year);
    bindir.push("bin");
    bindir.push("universal-darwin");
    return bindir;
}

#[cfg(target_os = "windows")]
pub fn binary_dir(year: i16) -> PathBuf {
    let mut bindir = base_dir(year);
    bindir.push("bin");
    bindir.push("windows");
    return bindir;
}


pub fn binary_string(year: i16, program: &str) -> String{
    let mut bin = binary_dir(year);
    bin.push(program);
    return bin.to_str().unwrap().to_string();
}

pub fn clone_bare_releases_repo() {
    let clone_path = crate::dirs::tinytex_repo();
    let url = "https://github.com/rstudio/tinytex-releases.git";
    if clone_path.read_dir().unwrap().next().is_none() {
        println!("Cloning TinyTeX releases repo");
        RepoBuilder::new()
            .bare(true)
            .clone(url, clone_path.as_path())
            .unwrap();
    }
}

pub fn get_latest_year() -> i16 {
    let latest = crate::tinytex::get_latest_version_number(None);
    let yyyymmdd = latest.clone().split_off(1);
    let yyyy = yyyymmdd.split_at(4).0;
    let latest_year = yyyy.parse::<i16>().unwrap();
    return latest_year;
}

pub fn setup_release_dirs() {
    let latest_year = get_latest_year();
    for year in 2020..(latest_year + 1) {
        let tt_dir = base_dir(year);
        create_dir_all(tt_dir).unwrap();
    }
}
