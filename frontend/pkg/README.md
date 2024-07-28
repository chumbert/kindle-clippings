# Kindle clippings exporter

This is an over-engineered exporter for Kindle clipping files born out of my own need and an
opportunity to productively procrastinate using Rust and WASM.

This tool is **boring software**:

1. Input your Kindle clippings file
2. Filter whatever you want to export
3. Select an export format and export

That's it.

No backend, no AI, no images, no login, no cookies, no tracking, no stats, no achievements, no streak.
Just a plain boring tool that works when you need it.

## Build frontend

```shell
wasm-pack build --target web
cp -r pkg frontend/
```

## Build and run cli

```shell
cargo run -- -h
```

## Roadmap

* Configurable export template (CLI)
* Configurable export template (Web UI)