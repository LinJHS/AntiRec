<script setup>
import { ref } from 'vue';

import axios from "axios";
import router from '../router'
import { exists, createDir, writeTextFile, BaseDirectory } from '@tauri-apps/api/fs';

const hasComplete = ref(false)
const displayMessage = ref('同步对抗扰动库中')

const baseURL = import.meta.env.VITE_BASE_URL;
const timeout = 10000;

// 将 UAP 写入文件
const writeUAP = async (data) => {
  if (!await exists('uap', { dir: BaseDirectory.AppData })) {
    console.log('create dir')
    await createDir('uap', { dir: BaseDirectory.AppData, recursive: true });
  }
  writeTextFile('uap/uap.ar', data, {
    dir: BaseDirectory.AppData
  });
  hasComplete.value = true
  displayMessage.value = '同步成功'
}

// 从远程获取 UAP 信息
const initUAP = async () => {
  axios.create({
    baseURL: baseURL,
    timeout: timeout,
  }).get('uap.ar')
    .then((res) => {
      if (res.status === 200) {
        writeUAP(res.data)
      } else {
        hasComplete.value = true
        displayMessage.value = '无法获取对抗扰动库'
      }
    })
    .catch((_err) => {
      router.push("/404");
    });

}

initUAP()

// 返回首页
const btnReturn = () => {
  router.push({ name: 'home' })
}

</script>

<template>
  <div class="container">
    <div class="title">
      <svg v-if="!hasComplete" xmlns="http://www.w3.org/2000/svg" width="32" height="32" viewBox="0 0 256 256">
        <path fill="#CDF1F6"
          d="M244 56v48a12 12 0 0 1-12 12h-48a12 12 0 1 1 0-24h17.1l-19-17.38c-.13-.12-.26-.24-.38-.37A76 76 0 1 0 127 204h1a75.53 75.53 0 0 0 52.15-20.72a12 12 0 0 1 16.49 17.45A99.45 99.45 0 0 1 128 228h-1.37a100 100 0 1 1 71.88-170.94L220 76.72V56a12 12 0 0 1 24 0Z" />
      </svg>
      <span>{{ hasComplete ? '同步成功' : '同步对抗扰动库中' }}</span>

    </div>
    <div class="return" @click="btnReturn">
      <svg xmlns="http://www.w3.org/2000/svg" width="1rem" height="1rem" viewBox="0 0 256 256">
        <path fill="#CDF1F6"
          d="M236 112a68.07 68.07 0 0 1-68 68H61l27.52 27.51a12 12 0 0 1-17 17l-48-48a12 12 0 0 1 0-17l48-48a12 12 0 1 1 17 17L61 156h107a44 44 0 0 0 0-88H80a12 12 0 0 1 0-24h88a68.07 68.07 0 0 1 68 68Z" />
      </svg>
      返回
    </div>
  </div>
</template>

<style scoped>
.container {
  display: flex;
  background-image: linear-gradient(90deg, #033e66, #001a32);
  width: 100vw;
  height: 100vh;
  align-items: center;
  justify-content: center;
}

@keyframes loading {
  0% {
    transform: rotate(0);
  }

  100% {
    transform: rotate(360deg);
  }
}

.title {
  display: flex;
  align-items: center;
  justify-content: center;
  color: #CDF1F6;
  font-size: 32px;

  svg {
    animation: loading 1s infinite linear;
  }

  span {
    margin-left: 10px;
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
