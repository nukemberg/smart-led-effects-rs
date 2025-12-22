# LED Simulator - WebAssembly Edition

A browser-based LED strip simulator that runs entirely in WebAssembly.

## Quick Start

```bash
# Build the WASM module and run the dev server
./build.sh && cargo run --bin serve --features serve
```

Or manually:

```bash
# Build WASM module
wasm-pack build --target web
cp -r pkg www/

# Run dev server
cargo run --bin serve --features serve
```

Then open `http://localhost:8080` in your browser.

## Building

First, install wasm-pack if you haven't already:
```bash
cargo install wasm-pack
```

Then build the WASM module:
```bash
wasm-pack build --target web
```

## Features

- Runs entirely in the browser with no backend server
- All LED effects available from the main library
- Interactive controls for effect selection, start/stop, and FPS adjustment
- Smooth animations using requestAnimationFrame
- Real-time rendering at up to 60 FPS
