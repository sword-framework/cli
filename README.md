# Sword CLI

> <img src="https://avatars.githubusercontent.com/u/228345998?s=200&v=4" align="right" width="120"/>

Structured web framework for rust built on top of axum.  
Designed to build server application with less boilerplate and more simplicity.  
It takes advantage of the tokio ecosystem to bring you performance with nice DX.

This cli provides a quick way to:

- Create a new sword project
- Add modules to an existing sword project
- Run a sword project

> Sword is in active development, expect breaking changes.  
> The CLI is in early stage expect bugs and missing features.

## Demo

> <img src="./assets/demo.gif" width="1000" />

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
