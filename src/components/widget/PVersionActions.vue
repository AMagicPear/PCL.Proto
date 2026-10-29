<script setup lang="ts">
import PSvgIcon from './PSvgIcon.vue'
import { pclText } from '@/api/pclLocale'
import { openMinecraftChangelog, saveMinecraftVersion } from '@/api/versionActions'

defineProps<{ versionId: string; manual?: boolean }>()
</script>

<template>
  <div class="version-actions">
    <button
      v-if="!manual"
      type="button"
      :title="pclText('Download.Version.SaveAs')"
      :aria-label="pclText('Download.Version.SaveAs')"
      @click="saveMinecraftVersion(versionId)"
    >
      <PSvgIcon name="lucide/save" :size="15" />
    </button>
    <button
      type="button"
      :title="pclText('Download.Version.Changelog')"
      :aria-label="pclText('Download.Version.Changelog')"
      @click="openMinecraftChangelog(versionId)"
    >
      <PSvgIcon name="lucide/info" :size="15" />
    </button>
    <button
      type="button"
      :title="pclText('Download.Version.DownloadServer')"
      :aria-label="pclText('Download.Version.DownloadServer')"
      @click="saveMinecraftVersion(versionId, true)"
    >
      <PSvgIcon name="lucide/server" :size="15" />
    </button>
  </div>
</template>

<style scoped>
.version-actions {
  display: flex;
  align-items: center;
  opacity: 0;
  pointer-events: none;
  transition: opacity 72ms;
}
.version-actions > button {
  width: 25px;
  height: 25px;
  padding: 5px;
  border: 0;
  border-radius: 50%;
  background: transparent;
  color: var(--color-tint-light, var(--color-tint));
  cursor: pointer;
  display: grid;
  place-items: center;
  transition:
    color 150ms,
    transform 250ms;
}
.version-actions > button:hover,
.version-actions > button:focus-visible {
  color: var(--color-tint-dark, var(--color-tint));
  transition-duration: 120ms, 250ms;
}
.version-actions > button:active {
  transform: scale(0.8);
}
.version-actions > button:focus-visible {
  outline: 2px solid var(--color-tint);
  outline-offset: 2px;
}
</style>
