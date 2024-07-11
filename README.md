# Axum with Askama Template Example
Demo axum with askama template. This repo show how to use askama with axum and some askama template example.

## Requirement
- Rust 1.79

## Installation (development)
1. Install cargo watch and systemfd `cargo install cargo-watch systemfd`
1. copy .env.example to .env then set ENVIRONTMENT=dev
1. run server in dev mode `systemfd --no-pid -s http::3000 -- cargo watch -x run`

## Testing
`cargo test`

## Deployment
1. copy .env.example to .env then set ENVIRONTMENT=prod
1. build project `cargo build --release`
1. run server `cargo run --release`
