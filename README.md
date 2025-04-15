# AppCUI-rs

# Web Terminal Implementation

This project implements a Web Terminal for TUI applications using Rust and WebAssembly. It leverages both WebGL and Canvas 2D rendering to display the terminal’s background, text, and cursor. The terminal also handles input events such as keyboard presses, mouse movements, clicks, and wheel events.

## Overview

The core of the terminal is implemented in Rust and exposes functionality using `wasm-bindgen`. Its features include:

- **Graphics Rendering:**  
  - **WebGL Canvas:** Renders the terminal background, cell colors, and simple graphical effects via shader programs.  
  - **2D Text Canvas:** Draws the terminal characters and the cursor using the Canvas 2D API.
- **Input Handling:**  
  - Listeners for keyboard, mouse (movement, button down/up), and mouse wheel events.
  - Uses Rust channels and a shared event queue to manage input events.
- **Dynamic Configuration:**  
  - Uses hidden HTML configuration elements to set terminal dimensions, font properties, and cell sizes.

## Prerequisites

Before you begin, make sure you have:

- **Rust Toolchain:**  
  Use the nightly toolchain, as this project requires unstable features.
- **wasm-bindgen:**  
  Add the following dependency in your `Cargo.toml`:
  ```toml
  wasm-bindgen = { version = "0.2" }
  ```
- **wasm-pack:**  
  Install [wasm-pack](https://rustwasm.github.io/wasm-pack/) for building your WebAssembly package.
- **A Web Server:**  
  Use the provided `server.py` or any static server to serve your files.

## Setup

### 1. Configure Rust for WebAssembly

Create or update your `.cargo/config.toml` to include the following target configuration:

```toml
[target.wasm32-unknown-unknown]
rustflags = [
    "-C", "target-feature=+atomics,+bulk-memory,+mutable-globals"
]

[unstable]
build-std = ["panic_abort", "std"]
```

This configuration enables atomic operations, bulk memory, and mutable globals on the `wasm32-unknown-unknown` target, and ensures that the build uses the required unstable std features.

### 2. Create a Library Package

Ensure your Rust project is set up as a library. In your library entry point, add the wasm-bindgen start macro to export your start function:

```rust
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen(start)]
pub fn start() {
    // Your initialization code, e.g., starting your TUI application.
    // For example:
    // app_cui::run();
}
```

Make sure that your library depends on the `appcui` crate (or your TUI framework) and that you use its features for rendering and input handling.

## Building the Package

Use `wasm-pack` to compile the project for the web target:

```sh
wasm-pack build --target web
```

Ensure that your Cargo project has the target `wasm32-unknown-unknown` installed. You can do so with:

```sh
rustup target add wasm32-unknown-unknown
```

## Example HTML File

Below is an example `index.html` that sets up the canvases and loads the compiled WebAssembly package. Save this file in your project’s root directory:

```html
<!DOCTYPE html>
<html>
<head>
  <meta charset="utf-8">
  <title>Web Terminal Test</title>
  <style>
    html, body {
      margin: 0;
      padding: 0;
      overflow: hidden;
    }
    #canvas, #textCanvas {
      position: absolute;
      top: 0;
      left: 0;
      width: 100%;
      height: 100%;
      display: block;
      background: transparent;
    }
    #textCanvas {
      pointer-events: none;
    }
    .config {
      display: none;
    }
  </style>
</head>
<body>
  <canvas id="canvas"></canvas>
  <canvas id="textCanvas"></canvas>

  <div class="config">
    <span id="terminal-cols">211</span>
    <span id="terminal-rows">56</span>
    <span id="terminal-font">Consolas Mono, monospace</span>
    <span id="terminal-font-size">20</span>
  </div>

  <script type="module">
    console.log("SharedArrayBuffer available:", typeof SharedArrayBuffer !== "undefined");
    import init, * as wasm from "./pkg/app_cui_test.js";

    init({ 
      module: new URL("./pkg/app_cui_test_bg.wasm", import.meta.url),
      memory: new WebAssembly.Memory({ initial: 200, maximum: 16384, shared: true })
    }).then(async () => {
      console.log("WASM module initialized");
      await wasm.initThreadPool(2);
    
      if (wasm.main) {
        wasm.main();
        console.log("main called");
      }
    });
  </script>
</body>
</html>
```

This file:
- Creates two canvases – one for WebGL background rendering and one for text rendering.
- Includes a hidden configuration section for terminal settings.
- Imports the WebAssembly package and initializes the thread pool.

## Running the Server

A simple Python server is provided to host the built files. Use the following `server.py`:

```python
import http.server
import socketserver
import os

class CustomHandler(http.server.SimpleHTTPRequestHandler):
    def send_head(self):
        path = self.translate_path(self.path)
        if os.path.isfile(path):
            f = open(path, 'rb')
            fs = os.fstat(f.fileno())
            self.send_response(200)
            if path.endswith('.js'):
                mime_type = "application/javascript"
            elif path.endswith('.wasm'):
                mime_type = "application/wasm"
            else:
                mime_type = "text/html"
            self.send_header("Content-Type", mime_type)
            self.send_header("Content-Length", str(fs.st_size))
            self.send_header("Cross-Origin-Opener-Policy", "same-origin")
            self.send_header("Cross-Origin-Embedder-Policy", "require-corp")
            self.end_headers()
            return f
        return super().send_head()

    def do_GET(self):
        f = self.send_head()
        if f:
            try:
                self.wfile.write(f.read())
            finally:
                f.close()

PORT = 4000
with socketserver.TCPServer(("", PORT), CustomHandler) as httpd:
    print(f"Serving on port {PORT}")
    httpd.serve_forever()
```

To run the server, execute:

```sh
python server.py
```

Then navigate to [http://localhost:4000/index.html](http://localhost:4000/index.html) in your browser to see your Web Terminal in action.

## Summary of Steps

1. **Set Up Your Rust Project:**  
   - Create a library package using your TUI framework (appcui).
   - Update `Cargo.toml` with the required dependency (`wasm-bindgen`).

2. **Configure the Toolchain:**  
   - Switch to the nightly Rust toolchain.
   - Create or update `.cargo/config.toml` as shown above.

3. **Add the wasm-bindgen Entry Point:**  
   - Annotate your library entry point with `#[wasm_bindgen(start)]`.

4. **Build for WebAssembly:**  
   - Ensure the target `wasm32-unknown-unknown` is installed.
   - Build using `wasm-pack build --target web`.

5. **Set Up the Web Environment:**  
   - Use the provided `index.html` as your webpage.
   - Launch a server using the provided `server.py`.

---

