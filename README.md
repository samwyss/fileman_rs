# fileman_rs

[![Crates.io](https://img.shields.io/crates/v/fileman_rs)](https://crates.io/crates/fileman_rs)
[![Docs.rs](https://img.shields.io/docsrs/fileman_rs)](https://docs.rs/crate/fileman_rs)
[![License](https://img.shields.io/crates/l/fileman_rs)](https://github.com/samwyss/fileman_rs)


## Purpose
A high-performance file management system for working with large quantities of files written in Rust.


## How to Use
Build using
~~~
cargo build --release
~~~
Run on your preferred command line followed by a desired task and arguments for said task

## All supported tasks
### Organize
Moves and organizes files from _./source_ directory into _./target_ directory based on files last modification date. NOTE: in the future this will be changed to the files creation date.
~~~
./fileman_rs organize ./source ./target
~~~


## How to Contribute
To report bugs/issues, please [submit an issue report](https://github.com/samwyss/fileman_rs/issues) using [this template](.github/templates/issue_report.md). To request features, please [submit a pull request](https://github.com/samwyss/fileman_rs/pulls) using [this template](./.github/templates/pull_request.md).

All current and future project information is stored in the [fileman_rs GitHub repository](https://github.com/samwyss/fileman_rs).

