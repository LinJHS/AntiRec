<script setup>
import router from '../router';
import { useI18n } from 'vue-i18n';
import Cookies from 'js-cookie';
import { ref } from 'vue';

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
    <!-- 替换 select 为自定义下拉菜单 -->
    <div class="lang-select" @click="isOpen = !isOpen">
      <span>{{ locale === 'zh' ? '中文' : 'English' }}</span>
      <div class="options" v-if="isOpen">
        <div class="option" @click="changeLanguage('zh')">中文</div>
        <div class="option" @click="changeLanguage('en')">English</div>
      </div>
    </div>
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

<script setup>
import { ref } from 'vue';

const isOpen = ref(false);

// 修改语言切换函数
const changeLanguage = (lang) => {
  locale.value = lang;
  Cookies.set('language', lang);
  isOpen.value = false;  // 选择后关闭下拉菜单
}
</script>

<style scoped>
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
  padding-right: 35px;
  /* 添加箭头 */
  &::after {
    content: '';
    position: absolute;
    right: 12px;
    top: 50%;
    transform: translateY(-50%);
    width: 10px;
    height: 10px;
    border-right: 2px solid #e96864;
    border-bottom: 2px solid #e96864;
    transform: translateY(-50%) rotate(45deg);
    transition: transform 0.3s ease;
  }

  &:hover {
    background: linear-gradient(135deg, #ec8c8933, #ec8c8922);
    box-shadow: 0 4px 20px #ec8c8922;
    color: #ec8c89;
    border-color: #ec8c8955;
  }

  .options {
    position: absolute;
    top: 100%;
    left: 0;
    right: 0;
    margin-top: 8px;
    background: rgba(255, 255, 255, 0.9);
    border-radius: 12px;
    overflow: hidden;
    box-shadow: 0 4px 20px rgba(0, 0, 0, 0.1);
    border: 1px solid #ec8c8933;

    .option {
      padding: 8px 20px;
      transition: all 0.2s ease;

      &:hover {
        background: rgba(233, 104, 100, 0.1);
      }
    }
  }
}
</style>