<script lang="tsx" setup>
import { pclText } from '@/api/pclLocale'
import { defineComponent, ref } from 'vue'
import PCard from './PCard.vue'
import PTextBox from './PTextBox.vue'
import PButton from './PButton.vue'
import type { MCPingResult, ExtraItem } from '@/types/mcPing'
import { isTauri, invoke } from '@tauri-apps/api/core'
import { error } from '@tauri-apps/plugin-log'
import sideTip from '@/composables/sideTip'

defineProps<{ embedded?: boolean }>()
const serverInput = ref<string>()
const mcPingResult = ref<MCPingResult | null>()
const latency = ref<number>()

async function performQuery() {
  try {
    let addrStr = serverInput.value?.trim()
    if (addrStr) {
      let [result, latency_got] = await invoke<[MCPingResult, number]>('server_query', {
        addrStr,
      })
      console.log('server_query', result)
      mcPingResult.value = result
      latency.value = latency_got
    } else {
      mcPingResult.value = null
      sideTip.show(pclText('Proto.Server.EnterAddress'))
    }
  } catch (err) {
    if (isTauri()) void error(`server query: ${err}`)
    mcPingResult.value = null
    sideTip.show(`${err}`, 'warn')
  }
}

function parseMinecraftText(text: string) {
  text = text.replace(/&/g, '&amp;').replace(/</g, '&lt;').replace(/>/g, '&gt;')
  const codes: Record<string, any> = {
    '0': { color: 'black' },
    '1': { color: 'dark_blue' },
    '2': { color: 'dark_green' },
    '3': { color: 'dark_aqua' },
    '4': { color: 'dark_red' },
    '5': { color: 'dark_purple' },
    '6': { color: 'gold' },
    '7': { color: 'gray' },
    '8': { color: 'dark_gray' },
    '9': { color: 'blue' },
    a: { color: 'green' },
    b: { color: 'aqua' },
    c: { color: 'red' },
    d: { color: 'light_purple' },
    e: { color: 'yellow' },
    f: { color: 'white' },
    l: { 'font-weight': 'bold' },
    m: { 'text-decoration': 'line-through' },
    n: { 'text-decoration': 'underline' },
    o: { 'font-style': 'italic' },
    r: {},
  }

  // 将单个样式对象转为 CSS 字符串
  const styleObjectToCss = (styleObj: Object) =>
    Object.entries(styleObj)
      .map(([property, value]) => `${property}: ${value}`)
      .join(';')

  let result = ''
  let i = 0 // scan over the whole string
  let currentStyle = {}
  while (i < text.length) {
    const char = text.charAt(i)
    // meet §
    if (char == '§') {
      result += '</span>'
      let styleRead = codes[text.charAt(++i)]
      if (styleRead) {
        if (styleRead.color) {
          // 遇到颜色代码，清除当前的样式，只记录颜色
          currentStyle = styleRead
        } else {
          // 遇到格式代码，附加当前的样式
          Object.assign(currentStyle, styleRead)
        }
        result += `<span style="${styleObjectToCss(styleRead)}">`
      } else {
        currentStyle = {}
      }
    } else {
      result += char
    }
    i++
  }
  result += '</span>'
  return result
}

// render description recursively
const DescriptionExtra = defineComponent(
  (props: { description: ExtraItem }) => {
    return () =>
      typeof props.description === 'string' ? (
        <span style={{ fontSize: '12px' }} v-html={parseMinecraftText(props.description)} />
      ) : (
        <>
          {props.description.extra?.map((item) => (
            <DescriptionExtra description={item} />
          ))}
          {props.description.text && (
            <span
              style={{
                color: props.description.color,
                fontWeight: props.description.bold ? 'bold' : 'unset',
                fontSize: '12px',
              }}
            >
              {props.description.text}
            </span>
          )}
        </>
      )
  },
  { name: 'DescriptionExtra', props: ['description'] },
)
</script>

<template>
  <component :is="embedded ? 'div' : PCard" :title="pclText('Tools.Test.ServerQuery.Title')">
    <div
      style="
        display: flex;
        gap: 16px;
        align-items: center;
        justify-content: space-around;
        margin-bottom: 8px;
      "
    >
      <PTextBox
        v-model="serverInput"
        :placeholder="pclText('Tools.ServerQuery.Address.Hint')"
        style="flex: 1"
      />
      <PButton inline @click="performQuery" :disabled="!serverInput">{{
        pclText('Tools.ServerQuery.Action.Query')
      }}</PButton>
    </div>
    <div class="server-card" v-if="mcPingResult">
      <div>
        <img class="server-favicon" v-if="mcPingResult.favicon" :src="mcPingResult.favicon" />
        <div class="server-title">
          <p style="font-size: 15px; font-weight: bold">
            {{ pclText('Proto.Server.Version', [mcPingResult.version.name]) }}
          </p>
          <!-- 描述部分需要递归渲染 -->
          <p>
            <DescriptionExtra :description="mcPingResult.description" />
          </p>
        </div>
      </div>
      <div class="server-info">
        <p>{{ mcPingResult.players.online }}/{{ mcPingResult.players.max }}</p>
        <p class="latency-text">{{ latency }}ms</p>
      </div>
    </div>
  </component>
</template>

<style scoped>
.server-card {
  background-image: url('@/assets/pictures/server_bg.png');
  background-size: auto;
  background-repeat: repeat-x;
  image-rendering: pixelated;
  border-radius: 4px;
  padding: 6px;
  color: white;
  display: flex;
  flex-direction: row;
  align-items: start;
  justify-content: space-between;
  gap: 10px;
}

.server-favicon {
  image-rendering: pixelated;
  float: left;
  margin-right: 10px;
}

.server-info {
  float: right;
}

.latency-text {
  color: rgba(155, 240, 11, 1);
}
</style>
