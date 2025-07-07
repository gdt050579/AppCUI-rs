console.log("SharedArrayBuffer available:", typeof SharedArrayBuffer !== "undefined");


import init, * as wasm from "./pkg/demo_licenta.js";

init({ 
  module: new URL("./pkg/demo_licenta.wasm", import.meta.url),
  memory: new WebAssembly.Memory({ initial: 2000, maximum: 16384, shared: true })
}).then(async () => {
  console.log("WASM module initialized");
  await wasm.initThreadPool(3);

  if (wasm.main) {
    wasm.main();
    console.log("wasm_main called");
  }
});