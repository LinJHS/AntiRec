<script setup>
import { invoke } from '@tauri-apps/api/tauri';
import { onBeforeUnmount, onMounted, ref } from 'vue';
import { exists, createDir, BaseDirectory, readTextFile } from '@tauri-apps/api/fs';

import router from '../router'
import { listen } from '@tauri-apps/api/event';


const uapDetected = ref(false)

// 启动防护
const startProtect = async () => {
}

var listenFromBackend
const canvasOriRef = ref()
const canvasNewRef = ref()

// 绘制波形
const drawWave = async (data, canvasRef) => {

}

// 初始化监听，获得后端返回的结果
const initListen = async () => {
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
