<script setup lang="ts">
import { computed, nextTick, onMounted, ref, watch } from 'vue'
import paths from '@/assets/vector-paths.json'
const props = defineProps<{ data: string }>()
const path = ref<SVGPathElement>()
const measured = ref('0 0 1024 1024')
const known = computed(() => Object.values(paths).find((item) => item.data === props.data))
const shape = computed(() => ({
  d: props.data.replace(/^F[01]\s*/, ''),
  'fill-rule': props.data.startsWith('F1') ? ('nonzero' as const) : ('evenodd' as const),
  ...(known.value?.style || { fill: 'currentColor' }),
}))
async function measure() {
  if (known.value) return
  await nextTick()
  const box = path.value?.getBBox()
  if (box && box.width && box.height)
    measured.value = `${box.x} ${box.y} ${box.width} ${box.height}`
}
onMounted(measure)
watch(() => props.data, measure)
</script>
<template>
  <svg class="pcl-path" :viewBox="known?.viewBox || measured" aria-hidden="true">
    <path ref="path" v-bind="shape" />
  </svg>
</template>
<style scoped>
.pcl-path {
  width: 1em;
  height: 1em;
  flex-shrink: 0;
}
</style>
