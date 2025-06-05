# Gaussian Splat Viewer

A 3d gaussian splat viewer built using Rust, WASM, and WebGPU.

## Building

1. Install the WebAssembly target:
```bash
rustup target add wasm32-unknown-unknown
```

2. Install wasm-pack:
```bash
curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh
```

3. Build the WebAssembly package:
```bash
wasm-pack build --target web --out-dir pkg
```

## Running

1. Start a local HTTP server:
```bash
python3 -m http.server 8000
```

2. Open your browser and navigate to:
```
http://localhost:8000
```

## Development

To make changes:

1. Edit the Rust code in `src/lib.rs`
2. Rebuild with `wasm-pack build --target web --out-dir pkg`
3. Refresh your browser
