import init from "./pkg/yewchat.js";

async function main() {
  const wasm = await init();  // init() mengembalikan InitOutput yang punya run_app()
  wasm.run_app();             // baru panggil run_app() dari objek itu
}

main();
