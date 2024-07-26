# Kindle clippings exported

This tool is **boring software**:
1. Input your Kindle clippings file
2. Filter whatever you want to export
3. Select an export format and export

That's it.
No backend, no AI, no images, no login, no cookies, no tracking, no stats, no achievements, no streak.
Just a plain boring tool that works when you need it.

## TODOs
 - [x] Bind DOM manipulation methods after module init using higher order functions
 - [x] Make sure filtering works
 - [x] Make basic exporter with Rust
 - [x] Make parsing work with my own clipping file
 - [x] Add CLI arguments for title and author filtering
 - [x] Make an export template for my own use
 - [x] Fix issue in UI where filter changes don't reset highlight list
 - [x] Style the content div a bit
 - [x] Have a look on mobile
 - [ ] Clean and publish ?
   - [ ] factor out stuff, rework quirky code
   - [ ] tests
   - [ ] CI
   - [ ] register A on clementhumbert.ch (kindle.clementhumbert.ch)
   - [ ] publish on GitHub sites
- [ ] Make export template configurable
  - [ ] In CLI
  - [ ] In web UI

## Build frontend

```shell
wasm-pack build --target web
cp -r pkg frontend/
```

## Build and run cli

```shell
cargo run <clippings file>
```
