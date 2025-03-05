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
  position: relative;
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

.lang-select {
  position: absolute;
  top: 20px;
  right: 20px;
  padding: 8px 20px;
  border-radius: 12px;
  background: linear-gradient(135deg, #ec8c8922, #ec8c8911);
  backdrop-filter: blur(10px);
  border: 1px solid #ec8c8933;
  color: #e96864;
  font-size: 1rem;
  font-weight: bold;
  cursor: pointer;
  transition: all 0.3s ease;
  appearance: none;
  -webkit-appearance: none;
  background-image: url("data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' width='12' height='12' viewBox='0 0 24 24' fill='none' stroke='%23e96864' stroke-width='2' stroke-linecap='round' stroke-linejoin='round'%3E%3Cpolyline points='6 9 12 15 18 9'%3E%3C/polyline%3E%3C/svg%3E");
  background-repeat: no-repeat;
  background-position: right 12px center;
  padding-right: 35px;

  &:hover {
    background: linear-gradient(135deg, #ec8c8933, #ec8c8922);
    box-shadow: 0 4px 20px #ec8c8922;
    color: #ec8c89;
    border-color: #ec8c8955;
  }

  option {
    background: white;
    color: #e96864;
    padding: 8px;
  }
}
</style>