# Yew WASM Counter Application
A basic counter application used while keeping track of loaded files to a test development environment. Built using RustLang and WASM.

## Build & Serve
### Dependencies
1. Wasm Pack: https://github.com/rustwasm/wasm-pack
2. Miniserve: https://github.com/svenstaro/miniserve

### Deploy
To build the project: `wasm-pack build --target web --out-name wasm --out-dir ./static`

To serve the project: `miniserve ./static --index index.html`
