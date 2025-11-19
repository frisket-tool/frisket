[![image](https://img.shields.io/badge/license-Apache%202.0%2CMIT-green
)](https://github.com/frisket-tool/frisket)
# Frisket a CLI tool for managing LaTeX projects

## Highlights
- Shareable texproject.toml containing information on dependencies
- Better LaTeX error handling, e.g. installation of missing packages
- A local `.texenv` containing project specific dependencies similar to pythons `.venv`

## Name
A frisket 'is a material that protects areas of work from unwanted changes'
used in printing presses, so it seemed a fitting choice for a program trying to prevent dependency issues when collaborating on LaTeX.


## Planned features
- Project templates, e.g. for specific journals
- Custom LaTeX package management, e.g. for local beamer themes not on CTAN
- Handling installation of TinyTeX directly by frisket

## Installation
frisket depends on TeXLive, the recommended installation is through TinyTeX-1 providing a minimal but usable set of LaTeX tools and packages.

For now frisket has to be build and installed by hand,
but we plan to release it as a crate in the future.

## Usage
frisket is still work in progress so functionality might be added to each command in the near future.

As a CLI tool it is run as
```
frisket <COMMAND>
```

use `frisket help` for the list of available commands and `frisket help <COMMAND>`


## Platform support

So far frisket is developed under Linux only,
but should work on Windows or MacOS as well.

## License
frisket is licensed under either of 

- Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or
  <https://www.apache.org/licenses/LICENSE-2.0>)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or <https://opensource.org/licenses/MIT>)

at your option.

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in uv
by you, as defined in the Apache-2.0 license, shall be dually licensed as above, without any
additional terms or conditions.
