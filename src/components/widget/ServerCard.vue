<script setup lang="ts">
import PCard from './PCard.vue'
import PButton from './PButton.vue'
import PListItem from './PListItem.vue'
import PSvgIcon from './PSvgIcon.vue'
import sideTip from '@/composables/sideTip'
import { pclText } from '@/api/pclLocale'
const props = defineProps<{
  name: string
  address: string
  icon?: string
  motd?: string
  players?: string
  latency?: number
}>()
const emit = defineEmits<{ connect: []; refresh: []; edit: []; remove: [] }>()
async function copyAddress() {
  try {
    await navigator.clipboard.writeText(props.address)
    sideTip.show(pclText('Instance.Server.Card.AddressCopied', [props.address]), 'success')
  } catch {
    sideTip.show(pclText('Instance.Server.Card.CopyAddressFailed'), 'warn')
  }
}
const fallback = `${import.meta.env.BASE_URL}pcl/images/Icons/NoIcon.png`
</script>
<template>
  <PCard hide-title class="server-entry"
    ><PListItem
      :title="name"
      :subtitle="`${latency == null ? '—' : `${latency} ms`} · ${players || '—'}`"
      :icon="icon || fallback"
      ><template #actions
        ><p class="motd">{{ motd }}</p>
        <PButton inline :click="() => emit('connect')" :title="name"
          ><PSvgIcon name="lucide/play" :size="18"
        /></PButton>
        <details>
          <summary :aria-label="pclText('Instance.Server.Card.Edit')">
            <PSvgIcon name="lucide/settings" :size="18" />
          </summary>
          <div class="server-menu">
            <PButton inline :click="() => emit('refresh')">{{
              pclText('Common.Action.Refresh')
            }}</PButton
            ><PButton inline :click="copyAddress">{{
              pclText('Instance.Server.Card.CopyAddress')
            }}</PButton
            ><PButton inline :click="() => emit('edit')">{{
              pclText('Instance.Server.Card.Edit')
            }}</PButton
            ><PButton inline type="warn" :click="() => emit('remove')">{{
              pclText('Instance.Server.Card.Remove')
            }}</PButton>
          </div>
        </details></template
      ></PListItem
    ></PCard
  >
</template>
<style scoped>
.server-entry {
  margin: 8px 0;
  overflow: visible;
}
.server-entry :deep(.mycard-content) {
  overflow: visible;
}
.server-entry :deep(.gameinfo-container) {
  min-height: 44px;
}
.server-entry :deep(img) {
  width: 34px;
  height: 34px;
  border-radius: 6px;
}
.server-entry :deep(.right) {
  display: flex;
  align-items: center;
  gap: 4px;
}
.motd {
  width: min(350px, 30vw);
  white-space: pre-line;
  font-size: 12px;
}
details {
  position: relative;
}
summary {
  cursor: pointer;
  list-style: none;
  display: flex;
  padding: 6px;
}
.server-menu {
  position: absolute;
  z-index: 3;
  right: 0;
  top: 100%;
  display: flex;
  flex-direction: column;
  min-width: 130px;
  padding: 6px;
  background: var(--color-background-soft);
  border-radius: 6px;
  box-shadow: var(--box-shadow);
}
.server-menu > a {
  padding: 6px;
  text-align: center;
  font-size: 13px;
}
</style>
