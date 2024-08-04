# linear

CLI client and TUI (terminal user interface) for [Linear](https://linear.app).

> [!WARNING]  
> This project is in very early development. Only a very basic set of features works at the moment.
> Contributions welcome!

## Features

- [ ] CLI
  - [ ] issue
    - [x] list
    - [ ] view
    - [ ] create
    - [ ] checkout
          Partial: implemented, but should allow interactive searching and
            checking existing linked PR that doesn't match the Linear-generated
            branch name.
  - [ ] project
    - [x] list
    - [ ] view
  - [ ] team
    - [x] list
    - [ ] view
- [ ] TUI

## Installation

### Install with cargo

If you have Rust installed, just run:

```bash
cargo install --locked --git https://github.com/theduke/linearly
```

## Usage

* [Install](#Installation)
* [Create a personal API token](https://linear.app/wasmer/settings/api)
* Save the the token:
  ```bash
  linear login
  ```
* Use the CLI:
  - `linear team list`
  - `linear project list`
  - `linear issue list [--team MYTEAM] [--assigne <USERNAME|me>] [--author <USERNAME|me>]`
    Note: you can use the special `me` filter for --assigne and --author,
          which will resolve to your current username.
  - `linear issue checkout MYTEAM-123`
    Create or check out a Git branch for a given issue.
    Uses the Linear-generated branch name.
