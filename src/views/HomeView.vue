<script setup>

import router from '../router';

// 各个按钮的返回
const btnStart = () => {
  router.push({ name: 'protect' })
}
const btnList = () => {
  router.push({ name: 'list' })
}
const btnAsync = () => {
  router.push({ name: 'async' })
}

</script>

<template>
  <div class="container">
    <div class="title">
      Anti Rec
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
  </div>
</template>

<style scoped>
.container {
  display: flex;
  background-image: linear-gradient(90deg, #033e66, #001a32);
  flex-direction: column;
  justify-content: center;
  align-items: center;
  width: 100vw;
  height: 100vh;

  .title {
    font-weight: bold;
    font-size: 6rem;
    background: linear-gradient(to right, #CDF1F6, #607de8);
    background-clip: text;
    -webkit-background-clip: text;
    color: transparent;
    border: 3px solid #0acbe033;
    padding: 10px 20px;
    border-radius: 30px;
    position: relative;

    .border-item {
      position: absolute;
      border: 3px solid #0acbe0cc;
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
    margin-top: 40px;
    font-size: 1.5rem;
    color: #CDF1F6ee;
    cursor: pointer;
    transition: all .25s;

    &:hover {
      color: #0acbe0cc;
      text-shadow:
        0 0 4px #0acbe088,
        0 0 10px #0acbe088,
        0 0 30px #0acbe088,
        0 0 50px #ffff0088;

    }
  }
}
</style>
