const wasm = import('./wasm/rust_wasm_vanilla');

console.log(wasm)
wasm.then(wasm => {
  console.log(wasm)
  wasm.run();
  wasm.greet('hello rust');
})