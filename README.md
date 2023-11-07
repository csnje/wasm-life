# Conway's Game of Life

## About

An implementation of [Conway's Game of Life](https://en.wikipedia.org/wiki/Conway%27s_Game_of_Life) in **Rust** **WebAssembly**.

![Image from Conway's Game of Life](./images/output.png)

## Prerequisites

Install [**Rust**](https://www.rust-lang.org/) and [**wasm-pack**](https://github.com/rustwasm/wasm-pack).

## Build

```bash
wasm-pack build --target web
```
or optimised for release
```bash
wasm-pack build --target web --release
```

## Run

Some options to serve the application include:
```bash
# Python 3.x
python3 -m http.server
# Python 2.x
python -m SimpleHTTPServer
# JDK 18 or later
jwebserver
```

Access via a web browser at [http://localhost:8000](http://localhost:8000).
