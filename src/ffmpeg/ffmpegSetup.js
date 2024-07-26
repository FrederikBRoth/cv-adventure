import { FFmpeg } from '/modules/@ffmpeg/ffmpeg/dist/esm/index.js';
import { fetchFile, toBlobURL } from '/modules/@ffmpeg/util/dist/esm/index.js';
export const loadFFmpeg = async () => {
  let ffmpeg = new FFmpeg()
  console.log(ffmpeg)
  if (!ffmpeg.loaded) {
    await ffmpeg.load({
      coreURL: await toBlobURL(`/modules/@ffmpeg/core/dist/esm/ffmpeg-core.js`, 'text/javascript'),
      wasmURL: await toBlobURL(`/modules/@ffmpeg/core/dist/esm/ffmpeg-core.wasm`, 'application/wasm'),
    });
  }
  console.log(ffmpeg)
  return ffmpeg;
};

export const transcodeVideo = async (inputFile) => {
  const ffmpeg = await loadFFmpeg();

  console.log("testerasds")

  await ffmpeg.writeFile('input.webm', await fetchFile('https://raw.githubusercontent.com/ffmpegwasm/testdata/master/Big_Buck_Bunny_180_10s.webm'));
  await ffmpeg.exec(['-i', 'input.webm', 'output.mp4']);
  const data = await ffmpeg.readFile('output.mp4');
  return new Blob([data.buffer], { type: 'video/mp4' });
};

