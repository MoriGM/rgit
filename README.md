# RGit

A Rust Web Server to show your git projects

## Build status

* GitHub-Actions:
	* [![Build Status](https://github.com/MoriGM/rgit/actions/workflows/rust.yml/badge.svg)](https://github.com/MoriGM/rgit/actions)

## Prerequisites

* Rust 1.60.0 and up
* Cargo

## Authors

* MoriGM [GitHub](https://github.com/MoriGM)

## License

This Project is licensed under the GNU GPL version 2

## Building

```
cargo build --release
```

## Test

### Tested Operating System

* Linux Manjaro

### Tested Architecture

* x86_64

## Built With

* [Eclipse](https://www.eclipse.org) - The IDE used
* [Git](https://git-scm.com) - Used Versioning Programm

## Setup

Create a file called `rgit_repos.toml` and register your git repos as following

```Toml
[[repos]]
name = "rgit"
path = "/home/git/repos/rgit"
```

After that run the server in the same folder as the config file and watch all your registered projects on the webpage.
