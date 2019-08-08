const wasm = import('./wasm/rust_wasm_vanilla.js');

wasm
  .then(wasm => {
    wasm.run();
  })
  .catch(err => {
    console.log(err);
  });