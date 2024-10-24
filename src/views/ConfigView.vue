<script setup>
import { ref } from 'vue';
import { writeTextFile, BaseDirectory } from '@tauri-apps/api/fs';
import router from '../router';

const appid = ref('');
const apiSecret = ref('');
const apiKey = ref('');
const enableASR = ref(true);

const saveConfig = async () => {
  const config = {
    xfyun: {
      appid: appid.value,
      apiSecret: apiSecret.value,
      apiKey: apiKey.value,
    },
    enableASR: enableASR.value,
  };

  try {
    await writeTextFile('config.json', JSON.stringify(config), { dir: BaseDirectory.AppConfig });
    router.push({ name: 'list' });
  } catch (error) {
    console.error('Error saving config:', error);
    // 处理保存错误
  }
};
</script>

<template>
  <div class="container">
    <h1>配置接口</h1>
    <form @submit.prevent="saveConfig">
      <div>
        <label for="appid">APPID:</label>
        <input id="appid" v-model="appid" required>
      </div>
      <div>
        <label for="apiSecret">API Secret:</label>
        <input id="apiSecret" v-model="apiSecret" required>
      </div>
      <div>
        <label for="apiKey">API Key:</label>
        <input id="apiKey" v-model="apiKey" required>
      </div>
      <div>
        <label>
          <input type="checkbox" v-model="enableASR">
          启�� ASR 识别
        </label>
      </div>
      <button type="submit">保存配置</button>
    </form>
  </div>
</template>

<style scoped>
.container {
  /* 添加适当的样式 */
}
</style>
