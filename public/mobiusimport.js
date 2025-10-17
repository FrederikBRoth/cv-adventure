import init from "/games/pkg/engine/engine_test.js";

export const initGame = () => {
  console.log("function called");
  init().then(() => {
    console.log("WASM Loaded");
  });
};
