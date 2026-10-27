<script setup lang="ts">
import { pclText } from '@/api/pclLocale'
import { defineAsyncComponent } from 'vue'
import { isTauri } from '@tauri-apps/api/core'
import PHint from '@/components/widget/PHint.vue'
import PCard from '@/components/widget/PCard.vue'
import { usePageSettings } from '@/composables/usePageSettings'
defineProps<{ launchLog?: string }>()
const { text } = usePageSettings('PageLaunchRight')
const website = !isTauri()
const ComponentShowcase = defineAsyncComponent(
  () => import('@/components/website/ComponentShowcase.vue'),
)
</script>
<template>
  <section class="pcl-page launch-cards" v-card-drop-children-animate>
    <PCard :title="pclText('Proto.About.Welcome')" default-fold-status="unfold"
      ><div class="proto-introduction">
        <PHint severity="info">{{ pclText('Proto.About.Intro') }}</PHint>
        <p>{{ pclText('Proto.About.Description') }}</p>
      </div></PCard
    >
    <PCard v-if="launchLog" :title="text('Launch.Right.Log.Title')">
      <pre class="launch-log">{{ launchLog }}</pre>
    </PCard>
    <ComponentShowcase v-if="website" />
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
