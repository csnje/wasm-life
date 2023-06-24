# Conway's Game of Life

## About

An implementation of [Conway's Game of Life](https://en.wikipedia.org/wiki/Conway%27s_Game_of_Life) in Rust WebAssembly.

![Image from Conway's Game of Life](./images/output.png)

## Prerequisites

Install [wasm-pack](https://github.com/rustwasm/wasm-pack).

## Compile

```bash
wasm-pack build --target web
```
or optimised for release
```bash
wasm-pack build --target web --release
```

## Serve and run

```bash
# Python 2.x
python -m SimpleHTTPServer
# Python 3.x
python3 -m http.server
```

Run in a browser at [http://localhost:8000](http://localhost:8000).
