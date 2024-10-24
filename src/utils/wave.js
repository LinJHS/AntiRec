/** wave.js
 * 
 * This file is a ported version of
 * https://github.com/mohayonao/wav-decoder
 * adapted to run within the Tauri project.
 */

var formats = {
  0x0001: "lpcm",
  0x0003: "lpcm",
};

// wave 文件解码
function waveDecoder(buffer, opts) {
  opts = opts || {};
  buffer = Uint8Array.from(buffer).buffer;
  //   if (ArrayBuffer && buffer instanceof ArrayBuffer) {
  //   }

  var dataView = new DataView(buffer);
  var reader = createReader(dataView);

  if (reader.string(4) !== "RIFF") {
    throw new TypeError("Invalid WAV file");
  }

  reader.uint32(); // skip file length

  if (reader.string(4) !== "WAVE") {
    throw new TypeError("Invalid WAV file");
  }

  var format = null;
  var audioData = null;

  do {
    var chunkType = reader.string(4);
    var chunkSize = reader.uint32();

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
  var formatId = reader.uint16();

  if (!formats.hasOwnProperty(formatId)) {
    return new TypeError(
      "Unsupported format in WAV file: 0x" + formatId.toString(16)
    );
  }

  var format = {
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

  var length = Math.floor(chunkSize / format.blockSize);
  var numberOfChannels = format.numberOfChannels;
  var sampleRate = format.sampleRate;
  var channelData = new Array(numberOfChannels);

  for (var ch = 0; ch < numberOfChannels; ch++) {
    channelData[ch] = new Float32Array(length);
    // channelData[ch] = new Uint8Array(length);
  }

  var retVal = readPCM(reader, channelData, length, format, opts);

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
  var bitDepth = format.bitDepth;
  var decoderOption = format.floatingPoint ? "f" : opts.symmetric ? "s" : "";
  var methodName = "pcm" + bitDepth + decoderOption;

  if (!reader[methodName]) {
    return new TypeError("Not supported bit depth: " + format.bitDepth);
  }

  var read = reader[methodName].bind(reader);
  var numberOfChannels = format.numberOfChannels;

  for (var i = 0; i < length; i++) {
    for (var ch = 0; ch < numberOfChannels; ch++) {
      channelData[ch][i] = read();
    }
  }

  return null;
}

// 创建 reader
function createReader(dataView) {
  var pos = 0;

  return {
    remain: function () {
      return dataView.byteLength - pos;
    },
    skip: function (n) {
      pos += n;
    },
    uint8: function () {
      var data = dataView.getUint8(pos, true);

      pos += 1;

      return data;
    },
    int16: function () {
      var data = dataView.getInt16(pos, true);

      pos += 2;

      return data;
    },
    uint16: function () {
      var data = dataView.getUint16(pos, true);

      pos += 2;

      return data;
    },
    uint32: function () {
      var data = dataView.getUint32(pos, true);

      pos += 4;

      return data;
    },
    string: function (n) {
      var data = "";

      for (var i = 0; i < n; i++) {
        data += String.fromCharCode(this.uint8());
      }

      return data;
    },
    pcm8: function () {
      var data = dataView.getUint8(pos) - 128;

      pos += 1;

      return data < 0 ? data / 128 : data / 127;
    },
    pcm8s: function () {
      var data = dataView.getUint8(pos) - 127.5;

      pos += 1;

      return data / 127.5;
    },
    pcm16: function () {
      var data = dataView.getInt16(pos, true);

      pos += 2;

      return data < 0 ? data / 32768 : data / 32767;
    },
    pcm16s: function () {
      var data = dataView.getInt16(pos, true);

      pos += 2;

      return data / 32768;
    },
    pcm24: function () {
      var x0 = dataView.getUint8(pos + 0);
      var x1 = dataView.getUint8(pos + 1);
      var x2 = dataView.getUint8(pos + 2);
      var xx = x0 + (x1 << 8) + (x2 << 16);
      var data = xx > 0x800000 ? xx - 0x1000000 : xx;

      pos += 3;

      return data < 0 ? data / 8388608 : data / 8388607;
    },
    pcm24s: function () {
      var x0 = dataView.getUint8(pos + 0);
      var x1 = dataView.getUint8(pos + 1);
      var x2 = dataView.getUint8(pos + 2);
      var xx = x0 + (x1 << 8) + (x2 << 16);
      var data = xx > 0x800000 ? xx - 0x1000000 : xx;

      pos += 3;

      return data / 8388608;
    },
    pcm32: function () {
      var data = dataView.getInt32(pos, true);

      pos += 4;

      return data < 0 ? data / 2147483648 : data / 2147483647;
    },
    pcm32s: function () {
      var data = dataView.getInt32(pos, true);

      pos += 4;

      return data / 2147483648;
    },
    pcm32f: function () {
      var data = dataView.getFloat32(pos, true);

      pos += 4;

      return data;
    },
    pcm64f: function () {
      var data = dataView.getFloat64(pos, true);

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

  var floatingPoint = !!(opts.floatingPoint || opts.float);
  var bitDepth = floatingPoint ? 32 : opts.bitDepth | 0 || 16;
  var bytes = bitDepth >> 3;
  var length = audioData.length * audioData.numberOfChannels * bytes;
  var dataView = new DataView(new Uint8Array(44 + length).buffer);
  var writer = createWriter(dataView);

  var format = {
    formatId: floatingPoint ? 0x0003 : 0x0001,
    floatingPoint: floatingPoint,
    numberOfChannels: audioData.numberOfChannels,
    sampleRate: audioData.sampleRate,
    bitDepth: bitDepth,
  };

  writeHeader(writer, format, dataView.buffer.byteLength - 8);

  var err = writeData(writer, format, length, audioData, opts);

  if (err instanceof Error) {
    throw err;
  }

  return dataView.buffer;
}

function toAudioData(data) {
  var audioData = {};

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
  var bytes = format.bitDepth >> 3;

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
  var bitDepth = format.bitDepth;
  var encoderOption = format.floatingPoint ? "f" : opts.symmetric ? "s" : "";
  var methodName = "pcm" + bitDepth + encoderOption;

  if (!writer[methodName]) {
    return new TypeError("Not supported bit depth: " + bitDepth);
  }

  var write = writer[methodName].bind(writer);
  var numberOfChannels = format.numberOfChannels;
  var channelData = audioData.channelData;

  writer.string("data");
  writer.uint32(length);

  for (var i = 0, imax = audioData.length; i < imax; i++) {
    for (var ch = 0; ch < numberOfChannels; ch++) {
      write(channelData[ch][i]);
    }
  }
}

function createWriter(dataView) {
  var pos = 0;

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
      for (var i = 0, imax = value.length; i < imax; i++) {
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

      var x0 = (value >> 0) & 0xff;
      var x1 = (value >> 8) & 0xff;
      var x2 = (value >> 16) & 0xff;

      dataView.setUint8(pos + 0, x0);
      dataView.setUint8(pos + 1, x1);
      dataView.setUint8(pos + 2, x2);
      pos += 3;
    },
    pcm24s: function (value) {
      value = Math.round(value * 8388608);
      value = Math.max(-8388608, Math.min(value, 8388607));

      var x0 = (value >> 0) & 0xff;
      var x1 = (value >> 8) & 0xff;
      var x2 = (value >> 16) & 0xff;

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
