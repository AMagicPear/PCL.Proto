<script setup lang="ts">
import PSvgIcon from './PSvgIcon.vue'
import PListActions from './PListActions.vue'
import { pclText } from '@/api/pclLocale'
import { openMinecraftChangelog, saveMinecraftVersion } from '@/api/versionActions'

defineProps<{ versionId: string; manual?: boolean }>()
</script>

<template>
  <PListActions>
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
  </PListActions>
</template>
