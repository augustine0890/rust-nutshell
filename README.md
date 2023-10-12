# Rust in a Nutshell
- This repository contains a collection of practice exercises and homework assignments for Rust programming language. These exercises are designed to help you learn Rust by practicing your skills and applying your knowledge to solve practical problems.
## Rustlang
- [Install](https://www.rust-lang.org/tools/install) Rust
- Config the `PATH` environment variable
  - `export PATH="$HOME/.cargo/bin:$PATH"`
  - `echo "export PATH='\$HOME/.cargo/bin:\$PATH'" >> ~/.zshrc`
  - `source ~/.zshrc`
  - `export PATH=$HOME/bin:/usr/local/bin:$PATH`
  then
  - `source ~/.zshrc`
- Check Rust version
  - `rustc --version`

## Projects
- `cargo new <project-name>` --> create a new project
- `cargo build` --> compile
- `cargo run` --> compile and run
- `cargo clean` --> remove generated `target`
- `cargo test some_module` --> test only for some module
- `cargo test some_module::test_example` --> if you have test named `test_example` (Specific Test Function)
## Acknowledgements
- This repository was inspired by similar collections of practice exercises and homework assignments for other programming languages. We would like to thank the authors of those repositories for their contributions and for inspiring us to create this repository for Rust.