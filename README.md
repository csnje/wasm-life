# Conway's Game of Life

## About

An implementation of [Conway's Game of Life](https://en.wikipedia.org/wiki/Conway%27s_Game_of_Life) in Rust WebAssembly.

## Compile

```bash
wasm-pack build --target web
```

## Serve and run

```bash
# Python 2.x
python -m SimpleHTTPServer
# Python 3.x
python3 -m http.server
```

Run in a browser at [localhost:8000](localhost:8000).
