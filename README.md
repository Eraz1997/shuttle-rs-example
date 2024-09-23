# Shuttle Rust Example

This is a simple URL shortener application built with shuttle.rs and Axum.

## Dev

```shell
cargo install cargo-shuttle
cargo shuttle run
cargo shuttle deploy
```

## Usage

```shell
# dev
curl -X POST http://localhost:8000 -H 'Content-Type: application/json' -d '{"url": "https://www.example.com"}'
curl http://localhost:8000/<id-from-previous-response>

# prod
curl -X POST https://shuttle-rs-example.shuttleapp.rs -H 'Content-Type: application/json' -d '{"url": "https://www.example.com"}'
curl https://shuttle-rs-example.shuttleapp.rs/<id-from-previous-response>
```