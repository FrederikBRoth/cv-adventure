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




async function normalizeTempo(info, id) {
}


async function combineAudioTracks(id, sessionID, name) {
  const audioDelayStr = "[1]adelay=8243[file_2];[2]adelay=12347[file_3];[3]adelay=28752[file_4];[4]adelay=32860[file_5];[5]adelay=36980[file_6];[6]adelay=53363[file_7];[7]adelay=57514[file_8];[8]adelay=61577[file_9]; \
            [0][file_2][file_3][file_4][file_5][file_6][file_7][file_8][file_9]amix=9:dropout_transition=1000, volume=10.0"

  return exec(ffmpeg,
    ['-i', "/img/basesong.mp3",
      '-i', ("./temp/d" + id + ".mp3"),
      '-i', ("./temp/d" + id + ".mp3"),
      '-i', ("./TempFiles/d" + id + ".mp3"),
      '-i', ("./TempFiles/d" + id + ".mp3"),
      '-i', ("./TempFiles/d" + id + ".mp3"),
      '-i', ("./TempFiles/d" + id + ".mp3"),
      '-i', ("./TempFiles/d" + id + ".mp3"),
      '-i', ("./TempFiles/d" + id + ".mp3"),
      '-filter_complex', audioDelayStr, ("./Output/" + name + sessionID + ".mp3"), "-y"])
}
export const getFileDuration = (file) => {
  return new Promise((resolve) => {
    const ctx = new AudioContext();
    var audioData = new ArrayBuffer(file.byteLength);
    new Uint8Array(audioData).set(new Uint8Array(file));
    ctx.decodeAudioData(audioData, data => {
      // this is the success callback
      const duration = data.duration;
      console.log('Audio file duration: ' + duration);
      resolve(duration);
    }, error => {
      // this is the error callback
      console.error(error);
      resolve(1.0);
    });

  })
}

export const transcodeVideo = async (name) => {
  const ffmpeg = await loadFFmpeg();
  const audioDelayStr = "[1]adelay=8243[file_2];[2]adelay=12347[file_3];[3]adelay=28752[file_4];[4]adelay=32860[file_5];[5]adelay=36980[file_6];[6]adelay=53363[file_7];[7]adelay=57514[file_8];[8]adelay=61577[file_9]; \
            [0][file_2][file_3][file_4][file_5][file_6][file_7][file_8][file_9]amix=9:dropout_transition=1000, volume=10.0"


  console.log("testerasds")
  let namefile = await fetchFile('https://frederikbroth.dk/api/getvoice?name=' + name);
  let duration = await getFileDuration(namefile);
  await ffmpeg.writeFile('name.mp3', namefile);

  const multiplier = duration / 0.60
  const str = "atempo=" + multiplier
  await ffmpeg.exec(['-i', 'name.mp3', '-filter_complex', str, 'namenormal.mp3', '-y'])
  await ffmpeg.writeFile('Michael.mp3', await fetchFile('/img/basesong.mp3'));

  await ffmpeg.exec(['-i', "Michael.mp3",
    '-i', 'namenormal.mp3',
    '-i', 'namenormal.mp3',
    '-i', 'namenormal.mp3',
    '-i', 'namenormal.mp3',
    '-i', 'namenormal.mp3',
    '-i', 'namenormal.mp3',
    '-i', 'namenormal.mp3',
    '-i', 'namenormal.mp3',
    '-filter_complex', audioDelayStr, 'finishedsong.mp3', "-y"])

  const data = await ffmpeg.readFile('finishedsong.mp3');
  return new Blob([data.buffer], { type: 'audo/mp3' });
};

