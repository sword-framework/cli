# Sword CLI

> <img src="./assets/demo.gif" width="500" align="right" />

Structured web framework for rust built on top of axum.

This cli provides a quick way to:

- Create a new sword project
- Add modules to an existing sword project
- Run a sword project

> Sword is in active development, expect breaking changes.
> The CLI is in early stage expect bugs and missing features.

## Installation

You can install the sword CLI using cargo:

```bash
cargo install sword-cli
```

Recommended: rename the binary to `sword` or `swd` for easier access:

```
# Bash/Zsh:
alias sword="sword-cli"

# Fish:
alias sword="sword-cli"

# PowerShell
Set-Alias sword sword-cli
```

## CLI use cases

### Create a new project

```bash
$ swd create
> What is your project name? my-sword-app
```
