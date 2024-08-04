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
  - `linear issue list [--team MYTEAM] [--assigne USERNAME]`
  - `linear team list`
  - `linear issue list`
  - ...
