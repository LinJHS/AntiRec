<script setup>

import router from '../router';

// Navigation functions for buttons
const btnStart = () => {
  router.push({ name: 'protect' }) // Navigate to protect page
}
const btnList = () => {
  router.push({ name: 'list' }) // Navigate to list page
}
const btnAsync = () => {
  router.push({ name: 'async' }) // Navigate to async page
}
const btnConfig = () => {
  router.push({ name: 'config' }) // Navigate to config page
}

</script>

<template>
  <div class="container">
    <div class="title">
      <img src="/images/logo-text.png" alt="Anti Rec" class="logo-img">
      <div class="border-item left_top"></div>
      <div class="border-item right_top"></div>
      <div class="border-item left_bottom"></div>
      <div class="border-item right_bottom"></div>
    </div>
    <div class="mode" @click="btnStart">
      启动 Anti Rec
    </div>
    <div class="mode" @click="btnList">
      查看历史通信记录
    </div>
    <div class="mode" @click="btnAsync">
      同步对抗扰动库
    </div>
    <div class="mode" @click="btnConfig">
      配置接口
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

  .title {
    position: relative;
    padding: 20px 40px;
    border-radius: 30px;
    background: rgba(255, 255, 255, 0.2);
    backdrop-filter: blur(10px);
    border: 1px solid rgba(255, 255, 255, 0.3);

    .logo-img {
      height: 100px;
      object-fit: contain;
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

  .mode {
    margin-top: 25px;
    font-size: 1.2rem;
    padding: 8px 20px;
    border-radius: 12px;
    background: linear-gradient(135deg, #ec8c8922, #ec8c8911);
    color: #e96864;
    font-weight: bold;
    cursor: pointer;
    transition: all 0.3s ease;
    border: 1px solid #ec8c8933;

    &:hover {
      transform: translateY(-2px);
      background: linear-gradient(135deg, #ec8c8933, #ec8c8922);
      box-shadow: 0 4px 20px #ec8c8922;
      color: #ec8c89;
      border-color: #ec8c8955;
    }
  }
}
</style>
