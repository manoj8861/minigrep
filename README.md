# MiniGrep using Rust
A simple Rust Cli app which can search by passing query and the file to be searched

## Build
```sh
cargo build
```
## Test

```sh
cargo test
```

## Run

```sh
cargo run -- <query> <filepath>
```
query - your search query
filepath - you file path

filepath - absolute 
or relative to app location you are running from

> Case Sensitive by default

Inorder to search without case sensitivity
```sh
IGNORE_CASE=1 cargo run -- <query> <filepath>
```
