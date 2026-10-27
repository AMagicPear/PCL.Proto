<script setup lang="ts">
import { onMounted, onUnmounted } from 'vue'
import PageSpeedLeft from './PageSpeedLeft.vue'
import PageSpeedRight from './PageSpeedRight.vue'
import { useFloatButton } from '@/composables/useFloatButton'
import { useTaskManager } from '@/stores/task'
const { floatButtonState } = useFloatButton()
const tasks = useTaskManager()
onMounted(() => (floatButtonState.visible = false))
onUnmounted(() => (floatButtonState.visible = tasks.activeTaskCount > 0))
</script>
<template>
  <div class="view-content"><PageSpeedLeft /><PageSpeedRight /></div>
</template>
