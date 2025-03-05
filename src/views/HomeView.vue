<script setup>
import router from '../router';
import { useI18n } from 'vue-i18n';
import Cookies from 'js-cookie';

const { locale } = useI18n();

// 语言切换函数
const changeLanguage = (lang) => {
  locale.value = lang;
  Cookies.set('language', lang);
}

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
    <!-- 添加语言选择器 -->
    <select class="lang-select" :value="locale" @change="changeLanguage($event.target.value)">
      <option value="zh">中文</option>
      <option value="en">English</option>
    </select>

    <div class="title">
      <img src="/images/logo-text.png" :alt="$t('title')" class="logo-img">
      <div class="border-item left_top"></div>
      <div class="border-item right_top"></div>
      <div class="border-item left_bottom"></div>
      <div class="border-item right_bottom"></div>
    </div>
    <div class="mode" @click="btnStart">
      {{ $t('home.btnStart') }}
    </div>
    <div class="mode" @click="btnList">
      {{ $t('home.btnList') }}
    </div>
    <div class="mode" @click="btnAsync">
      {{ $t('home.btnAsync') }}
    </div>
    <div class="mode" @click="btnConfig">
      {{ $t('home.btnConfig') }}
    </div>
  </div>
</template>

<style scoped>
.container {
  position: relative; /* 添加相对定位 */
  display: flex;
  background-image: linear-gradient(120deg, #fdfbfb 0%, #94c6e2 100%);
  flex-direction: column;
  justify-content: center;
  align-items: center;
  width: 100vw;
  height: 100vh;

  .title {
    position: relative;
    padding: 30px 50px;  /* 增加内边距 */
    border-radius: 35px;  /* 增加圆角 */
    background: rgba(255, 255, 255, 0.25);  /* 稍微增加透明度 */
    backdrop-filter: blur(12px);  /* 增加模糊效果 */
    border: 1px solid rgba(255, 255, 255, 0.4);  /* 边框更明显 */
    box-shadow: 0 8px 32px rgba(0, 0, 0, 0.1);  /* 添加阴影 */
    margin-bottom: 20px;  /* 增加底部间距 */

    .logo-img {
      height: 120px;  /* 稍微增大logo */
      object-fit: contain;
      transition: transform 0.3s ease;  /* 添加过渡效果 */

      &:hover {
        transform: scale(1.02);  /* 悬停时轻微放大 */
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

  .mode {
    margin-top: 20px;
    font-size: 1.25rem;
    padding: 12px 30px;  /* 增加按钮内边距 */
    border-radius: 15px;  /* 增加圆角 */
    background: linear-gradient(135deg, #ec8c8925, #ec8c8915);
    color: #e96864;
    font-weight: 600;
    cursor: pointer;
    transition: all 0.4s ease;
    border: 1px solid #ec8c8940;
    min-width: 200px;  /* 统一按钮宽度 */
    text-align: center;  /* 文字居中 */
    letter-spacing: 1px;  /* 增加字间距 */

    &:hover {
      transform: translateY(-3px);
      background: linear-gradient(135deg, #ec8c8940, #ec8c8925);
      box-shadow: 0 6px 25px #ec8c8930;
      color: #ec8c89;
      border-color: #ec8c8960;
    }

    &:active {
      transform: translateY(1px);  /* 点击效果 */
    }
  }
}

.lang-select {
  position: absolute;
  top: 25px;
  right: 25px;
  padding: 10px 15px;
  border-radius: 12px;
  background: rgba(255, 255, 255, 0.25);
  backdrop-filter: blur(12px);
  border: 1px solid rgba(255, 255, 255, 0.4);
  color: #e96864;
  font-size: 1rem;
  cursor: pointer;
  transition: all 0.3s ease;
  outline: none;  /* 移除默认outline */
  box-shadow: 0 4px 15px rgba(0, 0, 0, 0.05);  /* 添加阴影 */

  &:hover {
    background: rgba(255, 255, 255, 0.35);
    border-color: #ec8c8960;
    transform: translateY(-1px);  /* 悬停时轻微上浮 */
  }

  &:focus {
    border-color: #ec8c8980;
    box-shadow: 0 4px 20px rgba(236, 140, 137, 0.15);
  }
}
</style>
