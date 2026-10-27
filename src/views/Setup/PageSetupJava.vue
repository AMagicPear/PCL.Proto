<script setup lang="ts">
import PSvgIcon from '@/components/widget/PSvgIcon.vue'
import PCard from '@/components/widget/PCard.vue'

import { type IJavaRuntimeInfo, getJavaList, refreshJavaList } from '@/api/javaReq'
import { onMounted, ref } from 'vue'
import PListItem from '@/components/widget/PListItem.vue'
import PHint from '@/components/widget/PHint.vue'
import PButton from '@/components/widget/PButton.vue'
import { invoke, isTauri } from '@tauri-apps/api/core'
import sideTip from '@/composables/sideTip'
const loading = ref(false)
const error = ref('')
const javaList = ref<IJavaRuntimeInfo[]>([])
async function load(refresh = false) {
  if (!isTauri()) {
    error.value = '请在桌面客户端中读取或扫描 Java。'
    return
  }
  loading.value = true
  error.value = ''
  try {
    javaList.value = await (refresh ? refreshJavaList() : getJavaList())
  } catch (err) {
    error.value = '未能获取 Java 信息，请在桌面客户端中重试。'
    console.error(err)
  } finally {
    loading.value = false
  }
}
async function act(id: string, ..._args: string[]) {
  if (id !== 'BtnAdd') return
  if (!isTauri()) {
    sideTip.show('请在桌面客户端中添加 Java。', 'warn')
    return
  }
  try {
    const runtime = await invoke<IJavaRuntimeInfo | null>('add_java')
    if (runtime) {
      await load()
      sideTip.show('已添加 Java！', 'success')
    }
  } catch (err) {
    sideTip.show(`未能成功将 Java 加入列表中：${err}`, 'warn')
  }
}
onMounted(() => load())

import { useI18n } from 'vue-i18n'
import { pclText } from '@/api/pclLocale'
const { locale } = useI18n()
const text = (key: string) => pclText(key, locale.value)
</script>
<template>
  <section class="pcl-page">
    <div>
      <div data-control="PanBack">
        <div style="display: grid">
          <div
            data-control="PanMain"
            style="
              margin: 25px 25px 10px 25px;
              grid-row: 1 / span 1;
              grid-column: 1 / span 1;
              display: flex;
              flex-direction: column;
            "
          >
            <PCard style="margin: 0px 0px 15px 0px" hide-title>
              <div style="margin: 8px; display: flex; flex-direction: row; align-items: center">
                <PButton
                  data-control="BtnAdd"
                  :click="() => act('BtnAdd', 'BtnAdd_Click', '', '')"
                  inline
                  ><PSvgIcon name="lucide/circle-plus" :size="18" />{{
                    text('Setup.Java.Add')
                  }}</PButton
                >
              </div>
            </PCard>
            <PCard style="margin: 0px 0px 15px 0px" hide-title>
              <div style="margin: 15px; display: flex; flex-direction: column">
                <PHint v-if="error" severity="error">{{ error }}</PHint
                ><PListItem :title="text('Common.Option.Auto')" /><PListItem
                  v-for="runtime in javaList"
                  :key="runtime.directory_path"
                  :title="`${runtime.is_jdk ? 'JDK' : 'JRE'} ${runtime.slug_version} (${runtime.version}) ${runtime.architecture} ${runtime.implementor || ''}`"
                  :subtitle="runtime.directory_path"
                /><PButton inline :click="() => load(true)">{{
                  text('Common.Action.Refresh')
                }}</PButton>
              </div>
            </PCard>
          </div>
        </div>
      </div>
    </div>
  </section>
</template>
