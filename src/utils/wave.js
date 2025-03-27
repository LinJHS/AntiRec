/** wave.js
 * 
 * This file is a ported version of
 * https://github.com/mohayonao/wav-decoder
 * adapted to run within the Tauri project.
 */

let formats = {
  0x0001: "lpcm",
  0x0003: "lpcm",
};

// wave 文件解码
function waveDecoder(buffer, opts) {
  opts = opts || {};
  buffer = Uint8Array.from(buffer).buffer;
  //   if (ArrayBuffer && buffer instanceof ArrayBuffer) {
  //   }

  let dataView = new DataView(buffer);
  let reader = createReader(dataView);

  if (reader.string(4) !== "RIFF") {
    throw new TypeError("Invalid WAV file");
  }

  reader.uint32(); // skip file length

  if (reader.string(4) !== "WAVE") {
    throw new TypeError("Invalid WAV file");
  }

  let format = null;
  let audioData = null;

  do {
    let chunkType = reader.string(4);
    let chunkSize = reader.uint32();

    switch (chunkType) {
      case "fmt ":
        format = decodeFormat(reader, chunkSize);
        if (format instanceof Error) {
          throw format;
        }
        break;
      case "data":
        audioData = decodeData(reader, chunkSize, format, opts);
        if (audioData instanceof Error) {
          throw audioData;
        }
        break;
      default:
        reader.skip(chunkSize);
        break;
    }
  } while (audioData === null);

  return audioData;
}

// 解码格式
function decodeFormat(reader, chunkSize) {
  let formatId = reader.uint16();

  if (!formats.hasOwnProperty(formatId)) {
    return new TypeError(
      "Unsupported format in WAV file: 0x" + formatId.toString(16)
    );
  }

  let format = {
    formatId: formatId,
    floatingPoint: formatId === 0x0003,
    numberOfChannels: reader.uint16(),
    sampleRate: reader.uint32(),
    byteRate: reader.uint32(),
    blockSize: reader.uint16(),
    bitDepth: reader.uint16(),
  };
  reader.skip(chunkSize - 16);

  return format;
}

function decodeData(reader, chunkSize, format, opts) {
  chunkSize = Math.min(chunkSize, reader.remain());

  let length = Math.floor(chunkSize / format.blockSize);
  let numberOfChannels = format.numberOfChannels;
  let sampleRate = format.sampleRate;
  let channelData = new Array(numberOfChannels);

  for (let ch = 0; ch < numberOfChannels; ch++) {
    channelData[ch] = new Float32Array(length);
    // channelData[ch] = new Uint8Array(length);
  }

  let retVal = readPCM(reader, channelData, length, format, opts);

  if (retVal instanceof Error) {
    return retVal;
  }

  return {
    numberOfChannels: numberOfChannels,
    length: length,
    sampleRate: sampleRate,
    channelData: channelData,
  };
}

// 读取 PCM 格式代码
function readPCM(reader, channelData, length, format, opts) {
  let bitDepth = format.bitDepth;
  let decoderOption = format.floatingPoint ? "f" : opts.symmetric ? "s" : "";
  let methodName = "pcm" + bitDepth + decoderOption;

  if (!reader[methodName]) {
    return new TypeError("Not supported bit depth: " + format.bitDepth);
  }

  let read = reader[methodName].bind(reader);
  let numberOfChannels = format.numberOfChannels;

  for (let i = 0; i < length; i++) {
    for (let ch = 0; ch < numberOfChannels; ch++) {
      channelData[ch][i] = read();
    }
  }

  return null;
}

// 创建 reader
function createReader(dataView) {
  let pos = 0;

  return {
    remain: function () {
      return dataView.byteLength - pos;
    },
    skip: function (n) {
      pos += n;
    },
    uint8: function () {
      let data = dataView.getUint8(pos, true);

      pos += 1;

      return data;
    },
    int16: function () {
      let data = dataView.getInt16(pos, true);

      pos += 2;

      return data;
    },
    uint16: function () {
      let data = dataView.getUint16(pos, true);

      pos += 2;

      return data;
    },
    uint32: function () {
      let data = dataView.getUint32(pos, true);

      pos += 4;

      return data;
    },
    string: function (n) {
      let data = "";

      for (let i = 0; i < n; i++) {
        data += String.fromCharCode(this.uint8());
      }

      return data;
    },
    pcm8: function () {
      let data = dataView.getUint8(pos) - 128;

      pos += 1;

      return data < 0 ? data / 128 : data / 127;
    },
    pcm8s: function () {
      let data = dataView.getUint8(pos) - 127.5;

      pos += 1;

      return data / 127.5;
    },
    pcm16: function () {
      let data = dataView.getInt16(pos, true);

      pos += 2;

      return data < 0 ? data / 32768 : data / 32767;
    },
    pcm16s: function () {
      let data = dataView.getInt16(pos, true);

      pos += 2;

      return data / 32768;
    },
    pcm24: function () {
      let x0 = dataView.getUint8(pos + 0);
      let x1 = dataView.getUint8(pos + 1);
      let x2 = dataView.getUint8(pos + 2);
      let xx = x0 + (x1 << 8) + (x2 << 16);
      let data = xx > 0x800000 ? xx - 0x1000000 : xx;

      pos += 3;

      return data < 0 ? data / 8388608 : data / 8388607;
    },
    pcm24s: function () {
      let x0 = dataView.getUint8(pos + 0);
      let x1 = dataView.getUint8(pos + 1);
      let x2 = dataView.getUint8(pos + 2);
      let xx = x0 + (x1 << 8) + (x2 << 16);
      let data = xx > 0x800000 ? xx - 0x1000000 : xx;

      pos += 3;

      return data / 8388608;
    },
    pcm32: function () {
      let data = dataView.getInt32(pos, true);

      pos += 4;

      return data < 0 ? data / 2147483648 : data / 2147483647;
    },
    pcm32s: function () {
      let data = dataView.getInt32(pos, true);

      pos += 4;

      return data / 2147483648;
    },
    pcm32f: function () {
      let data = dataView.getFloat32(pos, true);

      pos += 4;

      return data;
    },
    pcm64f: function () {
      let data = dataView.getFloat64(pos, true);

      pos += 8;

      return data;
    },
  };
}

function waveEncoder(audioData, opts) {
  opts = opts || {};

  audioData = toAudioData(audioData);

  if (audioData === null) {
    throw new TypeError("Invalid AudioData");
  }

  let floatingPoint = !!(opts.floatingPoint || opts.float);
  let bitDepth = floatingPoint ? 32 : opts.bitDepth | 0 || 16;
  let bytes = bitDepth >> 3;
  let length = audioData.length * audioData.numberOfChannels * bytes;
  let dataView = new DataView(new Uint8Array(44 + length).buffer);
  let writer = createWriter(dataView);

  let format = {
    formatId: floatingPoint ? 0x0003 : 0x0001,
    floatingPoint: floatingPoint,
    numberOfChannels: audioData.numberOfChannels,
    sampleRate: audioData.sampleRate,
    bitDepth: bitDepth,
  };

  writeHeader(writer, format, dataView.buffer.byteLength - 8);

  let err = writeData(writer, format, length, audioData, opts);

  if (err instanceof Error) {
    throw err;
  }

  return dataView.buffer;
}

function toAudioData(data) {
  let audioData = {};

  if (typeof data.sampleRate !== "number") {
    return null;
  }
  if (!Array.isArray(data.channelData)) {
    return null;
  }
  if (!(data.channelData[0] instanceof Float32Array)) {
    return null;
  }

  audioData.numberOfChannels = data.channelData.length;
  audioData.length = data.channelData[0].length | 0;
  audioData.sampleRate = data.sampleRate | 0;
  audioData.channelData = data.channelData;

  return audioData;
}

function writeHeader(writer, format, length) {
  let bytes = format.bitDepth >> 3;

  writer.string("RIFF");
  writer.uint32(length);
  writer.string("WAVE");

  writer.string("fmt ");
  writer.uint32(16);
  writer.uint16(format.floatingPoint ? 0x0003 : 0x0001);
  writer.uint16(format.numberOfChannels);
  writer.uint32(format.sampleRate);
  writer.uint32(format.sampleRate * format.numberOfChannels * bytes);
  writer.uint16(format.numberOfChannels * bytes);
  writer.uint16(format.bitDepth);
}

function writeData(writer, format, length, audioData, opts) {
  let bitDepth = format.bitDepth;
  let encoderOption = format.floatingPoint ? "f" : opts.symmetric ? "s" : "";
  let methodName = "pcm" + bitDepth + encoderOption;

  if (!writer[methodName]) {
    return new TypeError("Not supported bit depth: " + bitDepth);
  }

  let write = writer[methodName].bind(writer);
  let numberOfChannels = format.numberOfChannels;
  let channelData = audioData.channelData;

  writer.string("data");
  writer.uint32(length);

  for (let i = 0, imax = audioData.length; i < imax; i++) {
    for (let ch = 0; ch < numberOfChannels; ch++) {
      write(channelData[ch][i]);
    }
  }
}

function createWriter(dataView) {
  let pos = 0;

  return {
    int16: function (value) {
      dataView.setInt16(pos, value, true);
      pos += 2;
    },
    uint16: function (value) {
      dataView.setUint16(pos, value, true);
      pos += 2;
    },
    uint32: function (value) {
      dataView.setUint32(pos, value, true);
      pos += 4;
    },
    string: function (value) {
      for (let i = 0, imax = value.length; i < imax; i++) {
        dataView.setUint8(pos++, value.charCodeAt(i));
      }
    },
    pcm8: function (value) {
      value = Math.max(-1, Math.min(value, +1));
      value = (value * 0.5 + 0.5) * 255;
      value = Math.round(value) | 0;
      dataView.setUint8(pos, value, true);
      pos += 1;
    },
    pcm8s: function (value) {
      value = Math.round(value * 128) + 128;
      value = Math.max(0, Math.min(value, 255));
      dataView.setUint8(pos, value, true);
      pos += 1;
    },
    pcm16: function (value) {
      value = Math.max(-1, Math.min(value, +1));
      value = value < 0 ? value * 32768 : value * 32767;
      value = Math.round(value) | 0;
      dataView.setInt16(pos, value, true);
      pos += 2;
    },
    pcm16s: function (value) {
      value = Math.round(value * 32768);
      value = Math.max(-32768, Math.min(value, 32767));
      dataView.setInt16(pos, value, true);
      pos += 2;
    },
    pcm24: function (value) {
      value = Math.max(-1, Math.min(value, +1));
      value = value < 0 ? 0x1000000 + value * 8388608 : value * 8388607;
      value = Math.round(value) | 0;

      let x0 = (value >> 0) & 0xff;
      let x1 = (value >> 8) & 0xff;
      let x2 = (value >> 16) & 0xff;

      dataView.setUint8(pos + 0, x0);
      dataView.setUint8(pos + 1, x1);
      dataView.setUint8(pos + 2, x2);
      pos += 3;
    },
    pcm24s: function (value) {
      value = Math.round(value * 8388608);
      value = Math.max(-8388608, Math.min(value, 8388607));

      let x0 = (value >> 0) & 0xff;
      let x1 = (value >> 8) & 0xff;
      let x2 = (value >> 16) & 0xff;

      dataView.setUint8(pos + 0, x0);
      dataView.setUint8(pos + 1, x1);
      dataView.setUint8(pos + 2, x2);
      pos += 3;
    },
    pcm32: function (value) {
      value = Math.max(-1, Math.min(value, +1));
      value = value < 0 ? value * 2147483648 : value * 2147483647;
      value = Math.round(value) | 0;
      dataView.setInt32(pos, value, true);
      pos += 4;
    },
    pcm32s: function (value) {
      value = Math.round(value * 2147483648);
      value = Math.max(-2147483648, Math.min(value, +2147483647));
      dataView.setInt32(pos, value, true);
      pos += 4;
    },
    pcm32f: function (value) {
      dataView.setFloat32(pos, value, true);
      pos += 4;
    },
  };
}

export { waveDecoder, waveEncoder };
