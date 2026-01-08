console.log("Loading Image Viewer Web Demo...");

import init from "./pkg/sixel_web.js";

init().then(() => {
  console.log("WASM module initialized successfully");
}).catch(error => {
  console.error("Failed to initialize WASM:", error);
});

