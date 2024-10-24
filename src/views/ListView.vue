<script setup>
import { onMounted, ref } from 'vue';
import router from '../router'
import WaveSurfer from 'wavesurfer.js'
import { exists, readDir, createDir, BaseDirectory, readBinaryFile, readTextFile, writeTextFile } from '@tauri-apps/api/fs';
import CryptoJS from 'crypto-js';
import { waveDecoder, waveEncoder } from '../utils/wave';
import config from '../config/config.json';

// 调用接口获得 ASR 结果
const xfIAT = async (waveFilepath, resultFilepath, resultDisplay) => {

  // 清空 resultDisplay
  resultDisplay.value = ''

  // 系统配置 
  const xfConfig = {
    // 请求地址
    hostUrl: "wss://iat-api.xfyun.cn/v2/iat",
    host: "iat-api.xfyun.cn",
    //从配置文件中获取
    appid: config.xfyun.appid,
    apiSecret: config.xfyun.apiSecret,
    apiKey: config.xfyun.apiKey,
    uri: "/v2/iat",
    highWaterMark: 1280
  }

  // 帧定义
  const FRAME = {
    STATUS_FIRST_FRAME: 0,
    STATUS_CONTINUE_FRAME: 1,
    STATUS_LAST_FRAME: 2
  }

  // 获取当前时间 RFC1123格式
  let date = (new Date().toUTCString())
  // 设置当前临时状态为初始化
  let status = FRAME.STATUS_FIRST_FRAME
  // 记录本次识别用sid
  let currentSid = ""
  // 识别结果
  let iatResult = []

  let wssUrl = xfConfig.hostUrl + "?authorization=" + getAuthStr(date) + "&date=" + date + "&host=" + xfConfig.host
  let ws = new WebSocket(wssUrl)

  // 连接建立完毕，读取数据进行识别
  ws.addEventListener('open', async (event) => {
    console.log("websocket connect!")
    initFileData()
  })

  // 得到识别结果后进行处理
  ws.addEventListener('message', (event) => {
    let res = JSON.parse(event.data)
    if (res.code != 0) {
      console.log(`error code ${res.code}, reason ${res.message}`)
      return
    }

    let str = ""
    if (res.data.status == 2) {
      // 最终识别结果
      console.log('最终识别结果')
      currentSid = res.sid
      ws.close()
    } else {
      // 否则是中间识别结果
      console.log('中间识别结果')
    }

    iatResult[res.data.result.sn] = res.data.result
    if (res.data.result.pgs == 'rpl') {
      console.log('动态修正')
      // 动态修正
      res.data.result.rg.forEach(i => {
        iatResult[i] = null
      })
    }
    iatResult.forEach(i => {
      if (i != null) {
        i.ws.forEach(j => {
          j.cw.forEach(k => {
            str += k.w
          })
        })
      }
    })
    console.log(str)
    resultDisplay.value = str
    if (res.data.status == 2) {
      // 如果是最终识别结果，还需要保存到文件中
      writeTextFile(resultFilepath, resultDisplay.value, {
        dir: BaseDirectory.AppData
      });
    }
  })

  // 资源释放
  ws.addEventListener('close', () => {
    console.log(`本次识别 sid ${currentSid}`)
    console.log('connect close!')
  })

  // 建连错误
  ws.addEventListener('error', (err) => {
    console.log("websocket connect err: " + err)
  })

  // 初始化需要传输的文件
  async function initFileData() {

    try {
      const waveFile = await readBinaryFile(waveFilepath, {
        dir: BaseDirectory.AppData
      });
      const waveDecodeData = waveDecoder(waveFile);

      // console.log(waveDecodeData)

      const { sampleRate, channelData } = waveDecodeData;
      var newChannelData = new Array(1);
      newChannelData[0] = new Float32Array(channelData[0].length);

      for (let i = 0; i < channelData[0].length; i++) {
        // 平均左右声道数据
        newChannelData[0][i] = (channelData[0][i] + channelData[1][i]) / 2
      }

      const newWaveFile = waveEncoder({
        numberOfChannels: 1,
        length: channelData[0].length,
        sampleRate: sampleRate,
        channelData: newChannelData,
      })

      let offset = 0;
      const chunkSize = xfConfig.highWaterMark;
      // console.log(monoData.buffer.byteLength)
      while (offset < newWaveFile.byteLength) {
        const chunk = newWaveFile.slice(offset, offset + chunkSize);
        // console.log(chunk)
        if (offset + chunkSize >= newWaveFile.byteLength)
          status = FRAME.STATUS_LAST_FRAME;
        send(chunk);
        offset += chunkSize;
      }
    } catch (error) {
      console.error('Error reading file:', error);
    }
  }

  // 鉴权签名
  function getAuthStr(date) {
    let signatureOrigin = `host: ${xfConfig.host}\ndate: ${date}\nGET ${xfConfig.uri} HTTP/1.1`
    let signatureSha = CryptoJS.HmacSHA256(signatureOrigin, xfConfig.apiSecret)
    let signature = CryptoJS.enc.Base64.stringify(signatureSha)
    let authorizationOrigin = `api_key="${xfConfig.apiKey}", algorithm="hmac-sha256", headers="host date request-line", signature="${signature}"`
    let authStr = CryptoJS.enc.Base64.stringify(CryptoJS.enc.Utf8.parse(authorizationOrigin))
    return authStr
  }

  // 传输数据
  function send(data) {

    // 将Uint8Array数据转换为Base64编码字符串
    function toBase64(uint8Array) {
      let binaryString = "";
      for (let i = 0; i < uint8Array.length; i++) {
        binaryString += String.fromCharCode(uint8Array[i]);
      }
      return btoa(binaryString);
    }

    let frame = "";
    let frameDataSection = {
      "status": status,
      "format": "audio/L16;rate=16000",
      "audio": toBase64(new Uint8Array(data)),
      "encoding": "raw"
    }
    switch (status) {
      case FRAME.STATUS_FIRST_FRAME:
        frame = {
          // 填充common
          common: {
            app_id: xfConfig.appid
          },
          //填充business
          business: {
            language: "zh_cn",
            domain: "iat",
            accent: "mandarin",
            dwa: "wpgs" // 可选参数，动态修正
          },
          //填充data
          data: frameDataSection
        }
        status = FRAME.STATUS_CONTINUE_FRAME;
        break;
      case FRAME.STATUS_CONTINUE_FRAME:
      case FRAME.STATUS_LAST_FRAME:
        //填充frame
        frame = {
          data: frameDataSection
        }
        break;
    }
    ws.send(JSON.stringify(frame))
  }
}



const wavePathList = ref([])

const wavePathIndex = ref(-1)

const wavesurferRefOri = ref()
const waveTextOri = ref('')
const wavesurferRefNew = ref()
const waveTextNew = ref('')

let wavesurferOri
let wavesurferNew

// 创建 WaveSurfer 实例
const createWaveSurfer = async () => {
  if (wavesurferOri) {
    wavesurferOri.destroy()
  }
  if (wavesurferNew) {
    wavesurferNew.destroy()
  }

  waveTextNew.value = ''
  waveTextOri.value = ''

  if (wavePathIndex.value === -1) {
    return
  }

  // ---------- ori ----------
  // 检查文件是否存在
  if (!await exists('waves/' + wavePathList.value[wavePathIndex.value] + '_ori.wav', {
    dir: BaseDirectory.AppData
  })) {
    console.log('file not exists')
    return
  }
  // 读取文件
  const waveFileOri = await readBinaryFile('waves/' + wavePathList.value[wavePathIndex.value] + '_ori.wav', {
    dir: BaseDirectory.AppData
  })

  // 处理 ASR 结果
  revealSpeechRecognition(wavePathList.value[wavePathIndex.value], 0);
  const waveFileBlobOri = new Blob([waveFileOri], { type: 'audio/wav' });
  const waveFileURLOri = window.URL.createObjectURL(waveFileBlobOri)

  // 创建 WaveSurfer 实例
  wavesurferOri = WaveSurfer.create({
    container: wavesurferRefOri.value,
    waveColor: '#0acbe0',
    progressColor: '#0acbe088',
    url: waveFileURLOri,
  })

  // 绑定点击事件
  wavesurferOri.on('click', () => {
    wavesurferOri.play()
  })
  // ---------- new ----------
  // 检查文件是否存在
  if (!await exists('waves/' + wavePathList.value[wavePathIndex.value] + '_new.wav', {
    dir: BaseDirectory.AppData
  })) {
    console.log('file not exists')
    return
  }
  // 读取文件
  const waveFileNew = await readBinaryFile('waves/' + wavePathList.value[wavePathIndex.value] + '_new.wav', {
    dir: BaseDirectory.AppData
  })
  // 处理 ASR 结果
  revealSpeechRecognition(wavePathList.value[wavePathIndex.value], 1);
  const waveFileBlobNew = new Blob([waveFileNew], { type: 'audio/wav' });
  const waveFileURLNew = window.URL.createObjectURL(waveFileBlobNew)

  // 创建 WaveSurfer 实例
  wavesurferNew = WaveSurfer.create({
    container: wavesurferRefNew.value,
    waveColor: '#0acbe0',
    progressColor: '#0acbe088',
    url: waveFileURLNew,
  })

  // 绑定点击事件
  wavesurferNew.on('click', () => {
    wavesurferNew.play()
  })
}

// 初始化，加载历史通讯列表
const init = async () => {
  if (!await exists('waves', { dir: BaseDirectory.AppData })) {
    console.log('create dir')
    await createDir('waves', { dir: BaseDirectory.AppData, recursive: true });
    // await writeFile('waves/config.json', { dir: BaseDirectory.AppData, recursive: true });
  } else {
    const entries = await readDir('waves', { dir: BaseDirectory.AppData, recursive: true });
    console.log('read dir')
    for (const entry of entries) {
      if (!entry.children) {
        // 通过正则表达式寻找文件
        const regex = /(.*)_new\.wav$/i;
        const match = entry.path.match(regex);
        if (match)
          wavePathList.value.push(entry.path.replace(/^.*[\\\/]/, '').replace(/_new\.wav$/, ''))
      }
    }
  }
  if (wavePathList.value.length !== 0) {
    wavePathIndex.value = 0
  }
}

onMounted(async () => {
  await init();
  createWaveSurfer();
})

const changeWaveSurfer = (index) => {
  wavePathIndex.value = index;
  createWaveSurfer();
}

const revealSpeechRecognition = async (filename, type) => {
  let fullTextFilepath
  let fullWaveFilepath
  let waveTextRef

  if (type === 0) {
    fullTextFilepath = 'waves/' + filename + '_ori.txt'
    fullWaveFilepath = 'waves/' + filename + '_ori.wav'
    waveTextRef = waveTextOri
  } else {
    fullTextFilepath = 'waves/' + filename + '_new.txt'
    fullWaveFilepath = 'waves/' + filename + '_new.wav'
    waveTextRef = waveTextNew
  }

  if (!await exists(fullTextFilepath, { dir: BaseDirectory.AppData })) {
    xfIAT(fullWaveFilepath, fullTextFilepath, waveTextRef)
  } else {
    const content = await readTextFile(fullTextFilepath, { dir: BaseDirectory.AppData })
    waveTextRef.value = content
  }
}

const btnReturn = () => {
  router.push({ name: 'home' })
}
</script>

<template>
  <div class="container">
    <div class="left">
      <div class="title">Anti Rec</div>
      <div class="subtitle">历史通讯列表</div>
      <!-- 一项都没有 -->
      <div v-if="wavePathIndex === -1" class="record-list">
      </div>
      <!-- 地址列表有内容 -->
      <div v-else class="record-list">
        <div v-for="(item, index) in wavePathList" :key="index" class="record-item"
          :class="wavePathIndex === index ? 'active' : ''" @click="changeWaveSurfer(index)">
          <div class="record-text">{{ item }}</div>
          <div v-if="wavePathIndex === index" class="record-border left_top"></div>
          <div v-if="wavePathIndex === index" class="record-border right_top"></div>
          <div v-if="wavePathIndex === index" class="record-border left_bottom"></div>
          <div v-if="wavePathIndex === index" class="record-border right_bottom"></div>
        </div>
      </div>
    </div>
    <div class="right">
      <div class="wave-ori">
        <div class="wave-title">原始音频波形与转录文本</div>
        <div class="wave-graph" ref="wavesurferRefOri"></div>
        <div class="wave-text">
          <div>{{ waveTextOri }}</div>
          <div class="wave-text-border left_top"></div>
          <div class="wave-text-border right_top"></div>
          <div class="wave-text-border left_bottom"></div>
          <div class="wave-text-border right_bottom"></div>
        </div>
      </div>
      <div class="wave-new">
        <div class="wave-title">防护后音频波形与识别结果</div>
        <div class="wave-graph" ref="wavesurferRefNew"></div>
        <div class="wave-text">
          <div>{{ waveTextNew }}</div>
          <div class="wave-text-border left_top"></div>
          <div class="wave-text-border right_top"></div>
          <div class="wave-text-border left_bottom"></div>
          <div class="wave-text-border right_bottom"></div>
        </div>
      </div>
    </div>
  </div>
  <div class="return" @click="btnReturn">
    <svg xmlns="http://www.w3.org/2000/svg" width="1rem" height="1rem" viewBox="0 0 256 256">
      <path fill="#CDF1F6"
        d="M236 112a68.07 68.07 0 0 1-68 68H61l27.52 27.51a12 12 0 0 1-17 17l-48-48a12 12 0 0 1 0-17l48-48a12 12 0 1 1 17 17L61 156h107a44 44 0 0 0 0-88H80a12 12 0 0 1 0-24h88a68.07 68.07 0 0 1 68 68Z" />
    </svg>
    返回
  </div>
</template>

<style scoped>
.container {
  display: flex;
  background-image: linear-gradient(90deg, #033e66, #001a32);
  width: 100vw;
  height: 100vh;
}

.left {
  width: 250px;
  background-image: linear-gradient(90deg, #033e6688, #04558c88);

  .title {
    color: #CDF1F6;
    font-size: 24px;
    margin: 10px;
    padding-bottom: 5px;
    font-weight: bold;
    border-bottom: 1px solid #0acbe0cc;
  }

  .subtitle {
    color: #CDF1F6;
    font-size: 16px;
    margin: 10px;
    font-weight: bold;
  }

  .record-list {
    margin: 10px;

    .record-item {
      padding: 0 10px;
      border: 1px solid #0acbe066;
      line-height: 30px;
      border-radius: 6px;
      position: relative;

      &~.record-item {
        margin-top: 5px;
      }

      .record-text {
        color: #0acbe0cc;
        font-size: 16px;
      }

      &.active .record-text {
        color: #CDF1F6;
        font-size: 16px;
      }

      .record-border {
        position: absolute;
        border: 2px solid #0acbe0cc;
        width: 3px;
        height: 3px;
      }

      .record-border.left_top {
        border-right: none;
        border-bottom: none;
        border-top-left-radius: 100%;
        left: 0;
        top: 0;
      }

      .record-border.right_top {
        border-left: none;
        border-bottom: none;
        border-top-right-radius: 100%;
        right: 0;
        top: 0;
      }

      .record-border.left_bottom {
        border-right: none;
        border-top: none;
        border-bottom-left-radius: 100%;
        left: 0;
        bottom: 0;
      }

      .record-border.right_bottom {
        border-left: none;
        border-top: none;
        border-bottom-right-radius: 100%;
        right: 0;
        bottom: 0;
      }
    }
  }
}

.right {
  flex: 1;
  margin: 10px;
  display: flex;
  flex-direction: column;

  .wave-ori {
    flex: 1;
    display: flex;
    flex-direction: column;
  }

  .wave-new {
    flex: 1;
    display: flex;
    flex-direction: column;
  }
}

.wave-title {
  line-height: 30px;
  color: #CDF1F6;
  font-size: 20px;
  margin: 0 auto;
  font-weight: bold;
}

.wave-graph {
  height: 128px;
}

.wave-text {
  flex: 1;
  padding: 0 10px;
  border: 1px solid #0acbe066;
  line-height: 30px;
  border-radius: 6px;
  position: relative;

  margin: 10px 0;

  color: #CDF1F6;
  font-size: 16px;

  .wave-text-border {
    position: absolute;
    border: 2px solid #0acbe0cc;
    width: 3px;
    height: 3px;
  }

  .wave-text-border.left_top {
    border-right: none;
    border-bottom: none;
    border-top-left-radius: 100%;
    left: 0;
    top: 0;
  }

  .wave-text-border.right_top {
    border-left: none;
    border-bottom: none;
    border-top-right-radius: 100%;
    right: 0;
    top: 0;
  }

  .wave-text-border.left_bottom {
    border-right: none;
    border-top: none;
    border-bottom-left-radius: 100%;
    left: 0;
    bottom: 0;
  }

  .wave-text-border.right_bottom {
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
  cursor: pointer;
  color: #CDF1F6;
}
</style>

