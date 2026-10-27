<script setup lang="ts">
import { pclIcons } from '@/components/icons/pclIcons'
withDefaults(defineProps<{ name: string; size?: number }>(), { size: 20 })
</script>

<template>
  <component
    :is="pclIcons[name]"
    v-if="pclIcons[name]"
    :size="size"
    :stroke-width="2"
    :fill="name === 'lucide/heart-filled' ? 'currentColor' : 'none'"
    class="pcl-icon"
    aria-hidden="true"
  />
</template>

<style scoped>
.pcl-icon {
  display: inline-block;
  flex-shrink: 0;
  vertical-align: middle;
}
</style>
