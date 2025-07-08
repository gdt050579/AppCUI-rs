# How to run this example
1. You must create a new Rust project with `cargo new --lib surface_load_web`
2. You need to build it with `wasm-pack build --target web` or `cargo build --target wasm32-unknown-unknown`
3. Host your application with `python server.py`
4. Go to `http://lolcahost:5000/index.html`