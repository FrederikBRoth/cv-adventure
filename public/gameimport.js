import init from "/games/pkg/tutorial8_depth.js";

export const initGame = () => {
  console.log("function called")
  init().then(() => {
    console.log("WASM Loaded");
  });
}; 
