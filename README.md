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

## Installation
For now frisket has to be build and installed by hand,
but we plan to release it as a crate in the future.

For this you need to install Rust and cargo and then run
```
cargo build
``` 
inside the checked out repository.
The binary will be in the `target/debug` subfolder and can be linked inside your bin folder to be available everywhere.

## Initial setup
After building and installing/linking frisket run `frisket setup` 
to prepare common directories and checkout metadata for the TinyTeX releases repository.

Afterwards, run `frisket install` to install the latest TinyTeX release
or `frisket install <year>` to install the latest release of each year.
Since TeXLive is a rolling release within each year it does not make sense to install older releases.

## Usage
frisket is still work in progress so functionality might be added to each command in the near future.

As a CLI tool it is run as
```
frisket <COMMAND>
```

use `frisket help` for the list of available commands and `frisket help <COMMAND>` for a specific subcommand.


## Platform support

So far frisket is developed under Linux only,
but should work on Windows or MacOS as well.

**Important** Installation and building for Windows is not finished or tested,
this will be done in a future release.

## Tab Autocompletion

frisket can now (>=v0.2.1) generate completions files for various shell tools.
Run one of the following commands depending on your shell to generate them

```bash
# Bash
frisket completions bash > ~/.local/share/bash-completion/completions/frisket

# Zsh
frisket completions zsh > ~/.zfunc/_frisket

# Fish
frisket completions fish > ~/.config/fish/completions/frisket.fish

# PowerShell - add to profile
frisket completions powershell >> $PROFILE

```

You might need to restart your shell for the changes to take effect.


## License
frisket is licensed under either of 

- Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or
  <https://www.apache.org/licenses/LICENSE-2.0>)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or <https://opensource.org/licenses/MIT>)

at your option.

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in frisket
by you, as defined in the Apache-2.0 license, shall be dually licensed as above, without any
additional terms or conditions.
