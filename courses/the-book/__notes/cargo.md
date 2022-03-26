# Cargo

- Cargo is the official tool for building and managing dependencies in Rust

## Common commands
- `cargo new my-project`
- `cargo build` (inside my-project) and `cargo run` (equivalent to `./target/debug/my-project`)
- `cargo run`
- `cargo build --release` to build for production

## Profiles
- Cargo automatically has two *profiles* defined, **debug** and **release**, which are used for development and production respectively
- debug has faster compilation but is not optimizeed in both perfomance and size
- release has a slower compilation but it is optimized in performance and size

## Cargo as Convention
Any cargo project at the root of its Git repository allows you to do this
```bash
git clone example.com/myproject
cd myproject
cargo build
```
