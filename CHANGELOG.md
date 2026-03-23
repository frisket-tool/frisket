# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).


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