# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.3.2] - 2026-04-21

Add option to init to older tex version.

### Added
- option `--year` for `frisket init` to specify an older texlive version

### Fixed
- `frisket init` on an existing directory without `texproject.toml` AND `main.tex` will not fail anymore.

## [0.3.1] - 2026-04-20

Fix MacOS errors & older TinyTeX installation

### Fixed

- remove platform specificity in use statement
- replace `xdg` crate by `directories` for os specific conventions
- older texlive manager installations now get updated repositories pointing to historic releases of texlive

## [0.3.0] - 2026-03-23

Breaking change: frisket now handles TinyTeX installation!
As a consequence you need to specify the `texlive_version` in `texproject.toml`

### Added

- Setup command to prepare data dir and get TinyTeX releases list
- Install command to install the most recent TinyTeX release per year
- This Changelog

### Removed

- Possibility to use preinstalled TeXLive! 

## [0.2.1] - 

### Added

- Completions command to generate tab autocompletion files


## [0.1.0] - 2025-11-19

Initial release