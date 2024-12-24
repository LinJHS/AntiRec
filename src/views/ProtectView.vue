<script setup>
import { invoke } from '@tauri-apps/api/tauri';
import { onBeforeUnmount, onMounted, ref } from 'vue';
import { exists, createDir, BaseDirectory, readTextFile } from '@tauri-apps/api/fs';

import router from '../router'
import { listen } from '@tauri-apps/api/event';

const uapDetected = ref(false);

// Start protection
const startProtect = async () => {
  if (!await exists('uap', { dir: BaseDirectory.AppData })) {
    // Create folder if it doesn't exist
    uapDetected.value = false;
    console.log('create dir');
    await createDir('uap', { dir: BaseDirectory.AppData, recursive: true });
  } else if (!await exists('uap/uap.ar', { dir: BaseDirectory.AppData })) {
    // UAP not found
    uapDetected.value = false;
  } else {
    // UAP found, load it
    uapDetected.value = true;
    const uapFileData = await readTextFile('uap/uap.ar', { dir: BaseDirectory.AppData });
    const lines = uapFileData.trim().split('\n');

    // Convert each line to a float and add to an array
    const floatArray = lines.map(line => {
      const num = parseFloat(line);
      if (!isNaN(num)) {
        return num;
      } else {
        console.warn(`"${line}" cannot be converted to a float.`);
        return null;
      }
    }).filter(num => num !== null); // Filter out failed conversions

    // Start audio protection
    invoke('audio_process', { addValues: floatArray.slice(0, 17000) });
  }
}

var listenFromBackend;
const canvasOriRef = ref();
const canvasNewRef = ref();

// Draw waveform
const drawWave = async (data, canvasRef) => {
  const context = canvasRef.value.getContext('2d');
  const width = canvasRef.value.width;
  const height = canvasRef.value.height;
  const step = data.length / width;
  context.strokeStyle = '#e96864';
  context.lineWidth = 2;
  context.clearRect(0, 0, width, height);
  context.beginPath();
  context.moveTo(0, height / 2);
  for (let i = 0; i < width; i++) {
    const sample = data[Math.ceil(i * step)];
    const y = sample * (height / 2) + (height / 2);
    context.lineTo(i, y);
  }
  context.stroke();
}

// Initialize listener to get results from backend
const initListen = async () => {
  listenFromBackend = await listen('audio_update', (event) => {
    drawWave(event.payload.ori, canvasOriRef);
    drawWave(event.payload.new, canvasNewRef);
  });
}

onMounted(() => {
  startProtect();
  initListen();
});

onBeforeUnmount(() => {
  listenFromBackend();
});

// Return to home page
const btnReturn = () => {
  router.push({ name: 'home' });
}

</script>

<template>
  <div class="container">
    <div class="wave-box">
      <div class="wave-container">
        <div class="wave-title">原始音频波形</div>
        <div class="wave-display">
          <canvas ref="canvasOriRef"></canvas>
          <div class="border-item left_top"></div>
          <div class="border-item right_top"></div>
          <div class="border-item left_bottom"></div>
          <div class="border-item right_bottom"></div>
        </div>
      </div>
      <div class="wave-container">
        <div class="wave-title">防护后音频波形</div>
        <div class="wave-display">
          <canvas ref="canvasNewRef"></canvas>
          <div class="border-item left_top"></div>
          <div class="border-item right_top"></div>
          <div class="border-item left_bottom"></div>
          <div class="border-item right_bottom"></div>
        </div>
      </div>
    </div>
    <div class="return" @click="btnReturn">
      <svg xmlns="http://www.w3.org/2000/svg" width="1rem" height="1rem" viewBox="0 0 256 256">
        <path fill="currentColor"
          d="M236 112a68.07 68.07 0 0 1-68 68H61l27.52 27.51a12 12 0 0 1-17 17l-48-48a12 12 0 0 1 0-17l48-48a12 12 0 1 1 17 17L61 156h107a44 44 0 0 0 0-88H80a12 12 0 0 1 0-24h88a68.07 68.07 0 0 1 68 68Z" />
      </svg>
      返回
    </div>
  </div>
</template>

<style scoped>
.container {
  padding-top: 20px;
  background-image: linear-gradient(120deg, #fdfbfb 0%, #94c6e2 100%);
  box-sizing: border-box;
  width: 100vw;
  height: 100vh;
}

.wave-box {
  width: 100%;
  max-width: 1200px;
  display: flex;
  flex-direction: column;
  /* margin: 10px; */
  gap: 10px;
}

.wave-container {
  flex: 1;
  display: flex;
  flex-direction: column;
  align-items: center;
}

.wave-title {
  color: #e96864;
  font-size: 1.5rem;
  font-weight: bold;
  margin-bottom: 15px;
  text-align: center;
}

.wave-display {
  position: relative;
  margin: 10px;
  width: calc(100% - 100px);
  padding: 20px;
  border-radius: 30px;
  background: rgba(255, 255, 255, 0.2);
  backdrop-filter: blur(10px);
  border: 1px solid #ec8c8933;
}

canvas {
  width: 100%;
  height: 25vh;
  /* background: linear-gradient(135deg, #ec8c8922, #ec8c8911); */
  border-radius: 12px;
  /* border: 1px solid #ec8c8933; */
}

.border-item {
  position: absolute;
  border: 3px solid #ec8c8933;
  width: 15px;
  height: 15px;
}

.left_top {
  border-right: none;
  border-bottom: none;
  border-top-left-radius: 100%;
  left: 0;
  top: 0;
}

.right_top {
  border-left: none;
  border-bottom: none;
  border-top-right-radius: 100%;
  right: 0;
  top: 0;
}

.left_bottom {
  border-right: none;
  border-top: none;
  border-bottom-left-radius: 100%;
  left: 0;
  bottom: 0;
}

.right_bottom {
  border-left: none;
  border-top: none;
  border-bottom-right-radius: 100%;
  right: 0;
  bottom: 0;
}

.return {
  position: absolute;
  left: 20px;
  bottom: 20px;
  padding: 8px 20px;
  border-radius: 12px;
  background: linear-gradient(135deg, #ec8c8922, #ec8c8911);
  color: #e96864;
  font-weight: bold;
  cursor: pointer;
  transition: all 0.3s ease;
  border: 1px solid #ec8c8933;
  display: flex;
  align-items: center;
  gap: 8px;
}

.return:hover {
  transform: translateY(-2px);
  background: linear-gradient(135deg, #ec8c8933, #ec8c8922);
  box-shadow: 0 4px 20px #ec8c8922;
  color: #ec8c89;
  border-color: #ec8c8955;
}
</style>
