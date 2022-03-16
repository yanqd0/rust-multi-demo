# rust-multi-demo

A Rust demo project for multiple targets.

There are 4 sub projects:

1. demo-lib: The main functions.
2. demo: The CLI binary wrapper.
3. demo-py: The Python interface wrapper.
4. demo-wasm: The web interface wrapper, for JS or TS.

## Library

```sh
cargo build
# or
cargo build -p demo-lib
```

## Binary

```sh
cargo run
# or
cargo run -p demo
```

## PyPI

```sh
maturin build -m src/demo-py/Cargo.toml
```

Run `pipx install maturin` first if it is not installed.

The wheel is built to `target/wheels/`.

## NPM

```sh
wasm-pack build src/demo-wasm
```

Run `cargo install wasm-pack` first if it is not installed.

The target files are built to `src/demo-wasm/pkg/`.

There is a `index.html` here for preview.

```sh
python3 -m http.server --directory src/demo-wasm/
```
