# Web Terminal

## Summary
The Web Terminal allows AppCUI applications to run in a web browser using WebAssembly, WebGL for rendering, and JavaScript for event handling.

## Prerequisites

Before you begin, make sure you have:

- **Rust Toolchain:**
  > [!IMPORTANT]
  > Use the nightly toolchain, as this project requires unstable features.
- **wasm-bindgen:**
  Add the following dependency in your `Cargo.toml`:
  ```toml
  wasm-bindgen = { version = "0.2" }
  ```
- **wasm-pack:**
  Install [wasm-pack](https://rustwasm.github.io/wasm-pack/) for building your WebAssembly package.
- **A Web Server:**
  Use the provided `server.py` (from the `test` directory in the repository) or any static server to serve your files.
  > [!WARNING]
  > If using threads, make sure to serve all your files in browser with these headers:
  > ```
  > Cross-Origin-Opener-Policy: "same-origin"
  > Cross-Origin-Embedder-Policy: "require-corp"
  > ```

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
    // your code
}
```

Make sure that your library depends on the `appcui` crate and that you use its features for rendering and input handling.

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

Below is an example `index.html` that sets up the canvases and loads the compiled WebAssembly package.

<details>
<summary> Index.html Example </summary>

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
    import init, * as wasm from "./pkg/your_application.js"; // Replace 'your_application' with your package name

    init({
      module: new URL("./pkg/your_application.wasm", import.meta.url), // Replace 'your_application'
      memory: new WebAssembly.Memory({ initial: 200, maximum: 16384, shared: true })
    }).then(async () => {
      console.log("WASM module initialized");
      // Example: Initialize a thread pool if your application uses threads
      // await wasm.initThreadPool(navigator.hardwareConcurrency); 

      if (wasm.start) { // Ensure your exported start function is called
        wasm.start();
        console.log("WASM start function called");
      }
    });
  </script>
</body>
</html>
```
</details>

This file:
- Creates two canvases: one for WebGL background rendering (`canvas`) and one for text rendering (`textCanvas`).
- Includes a hidden configuration section for terminal settings (cols, rows, font, font size). These values are read by the `WebTerminal` in `appcui`.
- Imports the WebAssembly package and initializes it. Make sure to replace `your_application` with the actual name of your wasm package.

## Running the Server

A simple Python server for hosting the application:
<details>
<summary>Python Server Example</summary>

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

</details>

To run the example Python server (assuming you are in the directory containing `server.py` and your `index.html` and `pkg` folder):

```sh
python server.py
```

Then navigate to `http://localhost:4000/index.html` (or the appropriate address and port for your server) in your browser.

## Implementation Details
The WebTerminal uses two HTML canvas elements:
- One canvas (`canvas`) is used for WebGL rendering of cell backgrounds. This allows for efficient rendering of colored backgrounds.
- A second canvas (`textCanvas`) is overlaid on top for rendering text characters.
- Event handling (keyboard, mouse) is done via JavaScript event listeners attached to the document, which then forward events to the Rust/WASM module.
- Configuration for terminal dimensions, font, etc., is typically read from hidden HTML elements on the page.

## Limitations
- Performance can vary depending on the browser and the complexity of the UI.
- Threading support relies on `SharedArrayBuffer`, which requires specific HTTP headers (`Cross-Origin-Opener-Policy: "same-origin"` and `Cross-Origin-Embedder-Policy: "require-corp"`) to be set by the web server.
- Clipboard integration uses the browser's asynchronous clipboard API.
