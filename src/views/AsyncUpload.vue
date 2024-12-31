<script setup>
import { ref } from 'vue';
import { writeTextFile, BaseDirectory } from '@tauri-apps/api/fs';

const dragActive = ref(false);
const uploadedFileName = ref('');
const uploadMessage = ref($t('async.uploadMessage'));

const handleFileUpload = async (event) => {
  const file = event.target.files?.[0] || event.dataTransfer?.files?.[0];
  if (!file) return;

  try {
    const reader = new FileReader();
    reader.onload = async (e) => {
      const fileContent = e.target.result;
      // 将文件内容保存到应用程序数据目录
      await writeTextFile('uap/uap.ar', fileContent, { dir: BaseDirectory.AppData });
      uploadedFileName.value = file.name;
      uploadMessage.value = $t('async.uploadSuccess');
    };
    reader.readAsText(file);
  } catch (error) {
    uploadMessage.value = $t('async.uploadError');
  }
};

const handleDragEnter = () => {
  dragActive.value = true;
};

const handleDragLeave = () => {
  dragActive.value = false;
};

const handleDragOver = (event) => {
  event.preventDefault();
};

const handleDrop = (event) => {
  event.preventDefault();
  dragActive.value = false;
  handleFileUpload(event);
};

// 返回首页
const btnReturn = () => {
  router.push({ name: 'home' });
};
</script>

<template>
  <div class="container">
    <div class="upload-title">
      <p class="upload-message">{{ uploadMessage }}</p>
      <div class="dropzone" :class="{ active: dragActive }" @dragenter="handleDragEnter" @dragleave="handleDragLeave"
        @dragover="handleDragOver" @drop="handleDrop" @click="$refs.fileInput.click()">
        <p v-if="!uploadedFileName">{{ $t('async.dropzone') }}</p>
        <p v-else>{{ $t('async.dropzoneAlready') }} {{ uploadedFileName }}</p>
        <input type="file" ref="fileInput" class="file-input" @change="handleFileUpload" style="display: none" />
      </div>
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
  flex-direction: column;
  align-items: center;
  justify-content: center;
  background-image: linear-gradient(120deg, #fdfbfb 0%, #94c6e2 100%);
  width: 100vw;
  height: 100vh;
}

.upload-title {
  display: flex;
  flex-direction: column;
  align-items: center;
  padding: 30px;
  border-radius: 20px;
  background: rgba(255, 255, 255, 0.2);
  backdrop-filter: blur(10px);
  border: 1px solid rgba(255, 255, 255, 0.3);
}

.upload-message {
  font-size: 20px;
  color: #e96864;
  font-weight: bold;
  margin-bottom: 20px;
  text-align: center;
}

.dropzone {
  width: 400px;
  height: 200px;
  border: 2px dashed #ec8c89;
  border-radius: 12px;
  display: flex;
  align-items: center;
  justify-content: center;
  color: #ec8c89;
  font-size: 18px;
  transition: all 0.3s ease;
  cursor: pointer;
}

.dropzone.active {
  background-color: rgba(236, 140, 137, 0.2);
  box-shadow: 0 4px 20px rgba(236, 140, 137, 0.2);
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
