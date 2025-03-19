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
    <div class="config-box">
      <div class="border-item left_top"></div>
      <div class="border-item right_top"></div>
      <div class="border-item left_bottom"></div>
      <div class="border-item right_bottom"></div>
      
      <h1>配置接口</h1>
      <form @submit.prevent="saveConfig">
        <div class="form-item">
          <label>
            <input type="checkbox" v-model="enableASR">
            启动 ASR 识别
          </label>
        </div>
        <template v-if="isASREnabled">
          <div class="form-item">
            <label for="appid">APPID:</label>
            <input id="appid" v-model="appid" required>
          </div>
          <div class="form-item">
            <label for="apiSecret">API Secret:</label>
            <input id="apiSecret" v-model="apiSecret" required>
          </div>
          <div class="form-item">
            <label for="apiKey">API Key:</label>
            <input id="apiKey" v-model="apiKey" required>
          </div>
        </template>
        <button type="submit">保存配置</button>
      </form>
    </div>
  </div>
</template>

<style scoped>
.container {
  display: flex;
  background-image: linear-gradient(120deg, #fdfbfb 0%, #94c6e2 100%);
  flex-direction: column;
  justify-content: center;
  align-items: center;
  width: 100vw;
  height: 100vh;
}

.config-box {
  position: relative;
  padding: 30px 40px;
  border-radius: 30px;
  background: rgba(255, 255, 255, 0.2);
  backdrop-filter: blur(10px);
  border: 1px solid rgba(255, 255, 255, 0.3);
  width: 400px;
}

h1 {
  text-align: center;
  color: #e96864;
  font-size: 24px;
  margin-bottom: 25px;
  font-weight: bold;
  padding-bottom: 15px;
  border-bottom: 1px solid #ec8c8933;
}

.form-item {
  margin-bottom: 20px;
  position: relative;
}

label {
  display: flex;
  align-items: center;
  color: #e96864;
  margin-bottom: 10px;
  font-size: 1rem;
  cursor: pointer;
}

input[type="text"],
input[type="password"],
input {
  width: 100%;
  padding: 12px 15px;
  border-radius: 12px;
  border: 1px solid #ec8c8966;
  background: rgba(255, 255, 255, 0.3);
  color: #e96864;
  font-size: 0.95rem;
  transition: all 0.3s ease;
}

input:focus {
  outline: none;
  border-color: #ec8c8988;
  box-shadow: 0 0 15px rgba(233, 104, 100, 0.1);
}

/* 优化多选框样式 */
input[type="checkbox"] {
  -webkit-appearance: none;
  appearance: none;
  width: 18px;
  height: 18px;
  border: 2px solid #ec8c8966;
  border-radius: 4px;
  margin-right: 10px;
  position: relative;
  cursor: pointer;
  transition: all 0.2s ease;
  padding: 0;
}

input[type="checkbox"]:checked {
  background-color: #e96864;
  border-color: #e96864;
}

input[type="checkbox"]:checked::after {
  content: "✓";
  position: absolute;
  color: white;
  font-size: 12px;
  left: 50%;
  top: 50%;
  transform: translate(-50%, -50%);
}

button {
  width: 100%;
  padding: 12px;
  border-radius: 12px;
  background: linear-gradient(135deg, #ec8c8966, #e9686466);
  color: white;
  border: none;
  font-size: 1.1rem;
  font-weight: bold;
  cursor: pointer;
  transition: all 0.3s ease;
  margin-top: 25px;
}

button:hover {
  transform: translateY(-2px);
  background: linear-gradient(135deg, #ec8c8988, #e9686488);
  box-shadow: 0 4px 15px rgba(233, 104, 100, 0.3);
}

.border-item {
  position: absolute;
  border: 2px solid #ec8c8933;
  width: 10px;
  height: 10px;
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
</style>
