import init from "/games/pkg/cv_game.js";

export const initGame = () => {
  console.log("function called")
  init().then(() => {
    console.log("WASM Loaded");
  });
}; 
