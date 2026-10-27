<script setup lang="ts">
import PHint from '@/components/widget/PHint.vue'
import PCard from '@/components/widget/PCard.vue'
import { usePageSettings } from '@/composables/usePageSettings'
defineProps<{ launchLog?: string }>()
const { text } = usePageSettings('PageLaunchRight')
</script>
<template>
  <section class="pcl-page launch-cards">
    <PCard title="欢迎来到 PCL.Proto！" default-fold-status="unfold"
      ><div class="proto-introduction">
        <PHint severity="info"
          >随着 PCL 的分支版本竞相启动，UI 的还原成为了一大困扰众开发者的难题。PCL.Proto
          应运而生。</PHint
        >
        <p>
          本项目以 PCL2（龙腾猫跃）和 PCL2-CE 为蓝本，旨在为各 PCL
          分支版本提供一个标准化的原型样本。该仓库使用 Vue3 搭建，如果你的仓库使用 Webview
          作为前端，则可以直接引用该项目。
        </p>
      </div></PCard
    >
    <PCard v-if="launchLog" :title="text('Launch.Right.Log.Title')">
      <pre class="launch-log">{{ launchLog }}</pre>
    </PCard>
  </section>
</template>
<style scoped>
.launch-cards {
  display: flex;
  flex-direction: column;
  gap: 15px;
  padding: 25px 25px 10px;
}
.launch-log {
  padding: 0 20px 18px;
  white-space: pre-wrap;
  overflow-wrap: anywhere;
  user-select: text;
}
</style>

<style scoped>
.proto-introduction {
  padding: 0 25px 15px;
  display: flex;
  flex-direction: column;
  gap: 10px;
}
</style>
