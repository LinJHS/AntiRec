<script setup>
import router from '../router'

const btnReturn = () => {
  router.push({ name: 'home' })
}

</script>

<template>
  <div class="container">
    <div class="title">
      <div class="text">404</div>
      <div class="subtitle">{{ $t('notfound.notfound') }}</div>
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
  flex-direction: column;
  justify-content: center;
  align-items: center;
}

.title {
  position: relative;
  padding: 30px 60px;
  border-radius: 30px;
  background: rgba(255, 255, 255, 0.2);
  backdrop-filter: blur(10px);
  border: 1px solid rgba(255, 255, 255, 0.3);
  text-align: center;

  .text {
    color: #e96864;
    font-size: 72px;
    font-weight: bold;
    line-height: 1.2;
  }

  .subtitle {
    color: #e96864;
    font-size: 24px;
    margin-top: 10px;
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
