/*
 * (C) Copyright 2025-2026 Jan Philipp Thiele
 * SPDX-License-Identifier: Apache-2.0 OR MIT
 */

use chrono::Datelike;
use git2::build::RepoBuilder;
use std::fs::{File, create_dir_all};
use std::io::Write;
use std::path::PathBuf;
use tar::Archive;
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

    let mut url: String = "".to_owned();
    if year > 2025 {
        url.push_str(&new_release_url(version.clone()));
    } else {
        url.push_str(&legacy_release_url(version.clone()));
    };
    let tmp_dir = Builder::new().prefix("frisket-tinytex").tempdir().unwrap();
    println!("Downloading TinyTeX {}", version);
    let release_fname = download(url, &tmp_dir);
    unpack_and_move(release_fname, year);
    if (year as i32) < chrono::Utc::now().year() {
        fix_older_texlive_repositories(year);
    }
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

pub fn fix_older_texlive_repositories(year: i16) {
    let bin = binary_string(year, "tlmgr");

    let mut historic_repo = "https://pi.kwarc.info/historic/systems/texlive/".to_string();
    historic_repo.push_str(year.to_string().as_str());
    historic_repo.push_str("/tlnet-final/");

    println!(
        "Setting up older TexLive version to use repo: {}",
        historic_repo.to_owned()
    );
    let output = std::process::Command::new(bin)
        .arg("option")
        .arg("repository")
        .arg(historic_repo.to_owned())
        .output()
        .expect("Error setting tlmgr repository!");

    if !output.status.success() {
        println!(
            "Something went wrong setting the repository option to {}",
            historic_repo
        );
        println!("{}", String::from_utf8(output.stdout).unwrap());
        println!("{}", String::from_utf8(output.stderr).unwrap());
    }
}

#[cfg(target_os = "linux")]
fn unpack_prefix() -> String {
    return ".TinyTeX".to_string();
}
#[cfg(target_os = "macos")]
fn unpack_prefix() -> String {
    return "TinyTeX".to_string();
}

#[cfg(any(target_os = "linux", target_os = "macos"))]
fn unpack_and_move_legacy(tar_gz: File, year: i16) {
    use flate2::read::GzDecoder;
    let tar = GzDecoder::new(tar_gz);
    let mut archive = Archive::new(tar);
    let tt_dir = tt_dir(year);
    archive
        .entries()
        .unwrap()
        .filter_map(|e| e.ok())
        .map(|mut entry| -> Result<PathBuf, Box<dyn std::error::Error>> {
            let path = entry
                .path()?
                .strip_prefix(unpack_prefix().as_str())?
                .to_owned();
            let mut dir = tt_dir.clone();
            dir.push(path);
            entry.unpack(&dir).unwrap();
            Ok(dir)
        })
        .filter_map(|e| e.ok())
        .for_each(|x| println!("> {}", x.display()));
}

#[cfg(any(target_os = "linux", target_os = "macos"))]
fn unpack_and_move_new(tar_xz: File, year: i16) {
    use xz::read::XzDecoder;
    let tt_dir = tt_dir(year);
    let tar = XzDecoder::new(tar_xz);
    let mut archive = Archive::new(tar);

    archive
        .entries()
        .unwrap()
        .filter_map(|e| e.ok())
        .map(|mut entry| -> Result<PathBuf, Box<dyn std::error::Error>> {
            let path = entry
                .path()?
                .strip_prefix(unpack_prefix().as_str())?
                .to_owned();
            let mut dir = tt_dir.clone();
            dir.push(path);
            entry.unpack(&dir).unwrap();
            Ok(dir)
        })
        .filter_map(|e| e.ok())
        .for_each(|x| println!("> {}", x.display()));
}

fn tt_dir(year: i16) -> PathBuf {
    let mut tt_dir = crate::dirs::data_home();
    tt_dir.push("TinyTeX");
    tt_dir.push(year.to_string());
    return tt_dir;
}

#[cfg(any(target_os = "linux", target_os = "macos"))]
fn unpack_and_move(fname: PathBuf, year: i16) {
    let mut tt_dir = crate::dirs::data_home();
    tt_dir.push("TinyTeX");
    tt_dir.push(year.to_string());
    println!("Installing into {}", tt_dir.clone().to_str().unwrap());
    let tar_gz = File::open(fname.as_path()).unwrap();
    if year > 2025 {
        unpack_and_move_new(tar_gz, year);
    } else {
        unpack_and_move_legacy(tar_gz, year);
    }
}

#[cfg(all(target_os = "linux", target_arch = "x86_64"))]
fn legacy_release_url(version: String) -> String {
    let result = [
        "https://github.com/rstudio/tinytex-releases/releases/download/",
        version.as_str(),
        "/TinyTeX-1-",
        version.as_str(),
        ".tar.gz",
    ]
    .join("");
    return result;
}
#[cfg(all(target_os = "linux", target_arch = "x86_64"))]
fn new_release_url(version: String) -> String {
    let result = [
        "https://github.com/rstudio/tinytex-releases/releases/download/",
        version.as_str(),
        "/TinyTeX-1-linux-x86_64-",
        version.as_str(),
        ".tar.xz",
    ]
    .join("");
    return result;
}

#[cfg(all(target_os = "linux", target_arch = "aarch64"))]
fn legacy_release_url(version: String) -> String {
    let result = [
        "https://github.com/rstudio/tinytex-releases/releases/download/",
        version.as_str(),
        "/TinyTeX-1-arm64-",
        version.as_str(),
        ".tar.gz",
    ]
    .join("");
    return result;
}

#[cfg(all(target_os = "linux", target_arch = "aarch64"))]
fn new_release_url(version: String) -> String {
    let result = [
        "https://github.com/rstudio/tinytex-releases/releases/download/",
        version.as_str(),
        "/TinyTeX-1-linux-arm64-",
        version.as_str(),
        ".tar.xz",
    ]
    .join("");
    return result;
}
#[cfg(target_os = "macos")]
fn legacy_release_url(version: String) -> String {
    let result = [
        "https://github.com/rstudio/tinytex-releases/releases/download/",
        version.as_str(),
        "/TinyTeX-1-",
        version.as_str(),
        ".tgz",
    ]
    .join("");
    return result;
}
#[cfg(target_os = "macos")]
fn new_release_url(version: String) -> String {
    let result = [
        "https://github.com/rstudio/tinytex-releases/releases/download/",
        version.as_str(),
        "/TinyTeX-1-darwin",
        version.as_str(),
        ".tar.xz",
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

pub fn binary_string(year: i16, program: &str) -> String {
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
