<script setup>
import { invoke } from '@tauri-apps/api/tauri';
import { onBeforeUnmount, onMounted, ref } from 'vue';
import { exists, createDir, BaseDirectory, readTextFile } from '@tauri-apps/api/fs';

import router from '../router'
import { listen } from '@tauri-apps/api/event';


const uapDetected = ref(false)

// 启动防护
const startProtect = async () => {
  if (!await exists('uap', { dir: BaseDirectory.AppData })) {
    // 不存在指定文件夹则创建
    uapDetected.value = false
    console.log('create dir')
    await createDir('uap', { dir: BaseDirectory.AppData, recursive: true });
  } else if (!await exists('uap/uap.ar', { dir: BaseDirectory.AppData })) {
    // 未找到 UAP
    uapDetected.value = false
  } else {
    // 找到 UAP 则加载
    uapDetected.value = true
    const uapFileData = await readTextFile('uap/uap.ar', { dir: BaseDirectory.AppData })
    const lines = uapFileData.trim().split('\n');

    // 转换每一行为浮点数并添加到数组中
    const floatArray = lines.map(line => {
      const num = parseFloat(line);
      // 确保转换成功且不是一个NaN值
      if (!isNaN(num)) {
        return num;
      } else {
        console.warn(`"${line}" 无法转换为浮点数.`);
        return null;
      }
    }).filter(num => num !== null); // 过滤掉转换失败的项

    // 启动音频防护
    invoke('audio_process', { addValues: floatArray.slice(0, 17000) });
  }
}

var listenFromBackend
const canvasOriRef = ref()
const canvasNewRef = ref()

// 绘制波形
const drawWave = async (data, canvasRef) => {
  const context = canvasRef.value.getContext('2d');
  const width = canvasRef.value.width;
  const height = canvasRef.value.height;
  const step = data.length / width;
  context.strokeStyle = '#CDF1F6';
  context.clearRect(0, 0, width, height);
  context.beginPath();
  context.moveTo(0, height / 2);
  for (let i = 0; i < width; i++) {
    const sample = data[Math.ceil(i * step)];
    const y = (sample) * (height / 2) + (height / 2);
    context.lineTo(i, y);
  }
  context.stroke();

}

// 初始化监听，获得后端返回的结果
const initListen = async () => {
  listenFromBackend = await listen('audio_update', (event) => {
    drawWave(event.payload.ori, canvasOriRef)
    drawWave(event.payload.new, canvasNewRef)
  });
}

onMounted(() => {
  startProtect()
  initListen()
})

onBeforeUnmount(() => {
  listenFromBackend();

})
const btnReturn = () => {
  router.push({ name: 'home' })
}

</script>

<template>
  <div class="container">
    <div class="title">
      原始音频波形
    </div>
    <div class="ori">
      <canvas ref="canvasOriRef"></canvas>

    </div>
    <div class="title">
      防护后音频波形
    </div>
    <div class="new">
      <canvas ref="canvasNewRef"></canvas>

    </div>
  </div>
  <div class="return" @click="btnReturn">
    <svg xmlns="http://www.w3.org/2000/svg" width="1rem" height="1rem" viewBox="0 0 256 256">
      <path fill="#CDF1F6"
        d="M236 112a68.07 68.07 0 0 1-68 68H61l27.52 27.51a12 12 0 0 1-17 17l-48-48a12 12 0 0 1 0-17l48-48a12 12 0 1 1 17 17L61 156h107a44 44 0 0 0 0-88H80a12 12 0 0 1 0-24h88a68.07 68.07 0 0 1 68 68Z" />
    </svg>
    返回
  </div>
</template>

<style scoped>
.container {
  display: flex;
  background-image: linear-gradient(90deg, #033e66, #001a32);
  width: 100vw;
  height: 100vh;
  flex-direction: column;
  justify-content: space-around;
  align-items: center;

  canvas {
    width: 80vw;
    height: 40vh;
  }

  .title {
    color: #CDF1F6;
    font-size: 28px;
  }
}

.return {
  position: absolute;
  left: 20px;
  bottom: 20px;
  cursor: pointer;
  color: #CDF1F6;
}
</style>
