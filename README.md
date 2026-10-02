# Actix Web starter

A minimal [Actix Web](https://actix.rs) service for the DayZer0 catalog briefs. It ships one
route — `GET /healthz` — so you can confirm the toolchain and the server before you build
anything on top of it.

## Stack

- Rust (edition 2021)
- [actix-web](https://crates.io/crates/actix-web) 4.x — the HTTP framework
- [serde](https://crates.io/crates/serde) + [serde_json](https://crates.io/crates/serde_json) — JSON

## Run it

```bash
cargo run
```

The server listens on `http://127.0.0.1:8080`. In another terminal:

```bash
curl http://127.0.0.1:8080/healthz
# {"status":"ok"}
```

## Build and check

```bash
cargo build
cargo fmt
cargo clippy
```

## Where to go next

Your board's first ticket points you here. Each later ticket adds a `/v1/...` endpoint — model
the request as a typed struct, validate it, and keep the business logic in plain functions you
can unit-test without HTTP. One ticket, one branch, one pull request with `Closes #N`.
