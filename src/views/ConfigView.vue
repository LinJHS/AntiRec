<script setup>
import { ref, computed, onMounted } from 'vue';
import { writeTextFile, readTextFile, BaseDirectory } from '@tauri-apps/api/fs';
import router from '../router';

const appid = ref('');
const apiSecret = ref('');
const apiKey = ref('');
const enableASR = ref(true);

const isASREnabled = computed(() => enableASR.value);

const saveConfig = async () => {
  const config = {
    xfyun: isASREnabled.value
      ? {
          appid: appid.value,
          apiSecret: apiSecret.value,
          apiKey: apiKey.value,
        }
      : null,
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

const loadConfig = async () => {
  try {
    const configContent = await readTextFile('config.json', { dir: BaseDirectory.AppConfig });
    const config = JSON.parse(configContent);
    
    enableASR.value = config.enableASR;
    if (config.xfyun) {
      appid.value = config.xfyun.appid;
      apiSecret.value = config.xfyun.apiSecret;
      apiKey.value = config.xfyun.apiKey;
    }
  } catch (error) {
    console.error('Error loading config:', error);
    // 如果配置文件不存在或读取失败，使用默认值
  }
};

onMounted(() => {
  loadConfig();
});
</script>

<template>
  <div class="container">
    <h1>配置接口</h1>
    <form @submit.prevent="saveConfig">
      <div>
        <label>
          <input type="checkbox" v-model="enableASR">
          启动 ASR 识别
        </label>
      </div>
      <template v-if="isASREnabled">
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
      </template>
      <button type="submit">保存配置</button>
    </form>
  </div>
</template>

<style scoped>
.container {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  background-image: linear-gradient(90deg, #033e66, #001a32);
  width: 100vw;
  height: 100vh;
  color: #CDF1F6;
}

h1 {
  font-size: 24px;
  margin-bottom: 20px;
  font-weight: bold;
}

form {
  background-image: linear-gradient(90deg, #033e6688, #04558c88);
  padding: 20px;
  border-radius: 6px;
  width: 300px;
}

form div {
  margin-bottom: 15px;
}

label {
  display: block;
  margin-bottom: 5px;
  font-size: 16px;
}

input[type="text"],
input[type="password"] {
  width: 100%;
  padding: 8px;
  border: 1px solid #0acbe066;
  border-radius: 4px;
  background-color: rgba(205, 241, 246, 0.1);
  color: #CDF1F6;
}

input[type="checkbox"] {
  margin-right: 5px;
}

button {
  width: 100%;
  padding: 10px;
  background-color: #0acbe0;
  color: #001a32;
  border: none;
  border-radius: 4px;
  font-size: 16px;
  cursor: pointer;
  transition: background-color 0.3s;
}

button:hover {
  background-color: #08a9b9;
}

.form-border {
  position: absolute;
  border: 2px solid #0acbe0cc;
  width: 3px;
  height: 3px;
}

.form-border.left_top {
  border-right: none;
  border-bottom: none;
  border-top-left-radius: 100%;
  left: 0;
  top: 0;
}

.form-border.right_top {
  border-left: none;
  border-bottom: none;
  border-top-right-radius: 100%;
  right: 0;
  top: 0;
}

.form-border.left_bottom {
  border-right: none;
  border-top: none;
  border-bottom-left-radius: 100%;
  left: 0;
  bottom: 0;
}

.form-border.right_bottom {
  border-left: none;
  border-top: none;
  border-bottom-right-radius: 100%;
  right: 0;
  bottom: 0;
}
</style>
