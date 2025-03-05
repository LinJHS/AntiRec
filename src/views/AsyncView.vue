<script setup>
import { ref } from 'vue';

import axios from "axios";
import router from '../router'
import { exists, createDir, writeTextFile, BaseDirectory } from '@tauri-apps/api/fs';

import { useI18n } from 'vue-i18n';
const { t } = useI18n();

const hasComplete = ref(false)
const displayMessage = ref(t('async.displaySync'))

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
  displayMessage.value = t('async.displaySyncSuccess')
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
        displayMessage.value = t('async.displaySyncFail')
      }
    })
    .catch((_err) => {
      router.push("/async-upload");
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
      <div class="content">
        <svg v-if="!hasComplete" xmlns="http://www.w3.org/2000/svg" width="32" height="32" viewBox="0 0 256 256">
          <path fill="currentColor"
            d="M244 56v48a12 12 0 0 1-12 12h-48a12 12 0 1 1 0-24h17.1l-19-17.38c-.13-.12-.26-.24-.38-.37A76 76 0 1 0 127 204h1a75.53 75.53 0 0 0 52.15-20.72a12 12 0 0 1 16.49 17.45A99.45 99.45 0 0 1 128 228h-1.37a100 100 0 1 1 71.88-170.94L220 76.72V56a12 12 0 0 1 24 0Z" />
        </svg>
        <span>{{ displayMessage }}</span>
      </div>
      <div class="border-item left_top"></div>
      <div class="border-item right_top"></div>
      <div class="border-item left_bottom"></div>
      <div class="border-item right_bottom"></div>
    </div>
    <div class="return" @click="btnReturn">
      <svg xmlns="http://www.w3.org/2000/svg" width="1rem" height="1rem" viewBox="0 0 256 256">
        <path fill="currentColor"
          d="M236 112a68.07 68.07 0 0 1-68 68H61l27.52 27.51a12 12 0 0 1-17 17l-48-48a12 12 0 0 1 0-17l48-48a12 12 0 1 1 17 17L61 156h107a44 44 0 0 0 0-88H80a12 12 0 0 1 0-24h88a68.07 68.07 0 0 1 68 68Z" />
      </svg>
      {{ $t('return') }} </div>
  </div>
</template>

<style scoped>
.container {
  display: flex;
  background-image: linear-gradient(120deg, #fdfbfb 0%, #94c6e2 100%);
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
  position: relative;
  padding: 30px 60px;
  border-radius: 30px;
  background: rgba(255, 255, 255, 0.2);
  backdrop-filter: blur(10px);
  border: 1px solid rgba(255, 255, 255, 0.3);

  .content {
    display: flex;
    align-items: center;
    justify-content: center;
    color: #e96864;
    font-size: 24px;
    font-weight: bold;

    svg {
      animation: loading 1s infinite linear;
      margin-right: 12px;
    }
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

  &:hover {
    transform: translateY(-2px);
    background: linear-gradient(135deg, #ec8c8933, #ec8c8922);
    box-shadow: 0 4px 20px #ec8c8922;
    color: #ec8c89;
    border-color: #ec8c8955;
  }

  svg {
    fill: currentColor;
  }
}
</style>
