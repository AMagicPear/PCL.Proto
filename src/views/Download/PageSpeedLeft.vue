<script setup lang="ts">
import { computed, onMounted } from 'vue'
import { useTaskManager } from '@/stores/task'
import useSideNavState from '@/stores/windowState'
import { pclText } from '@/api/pclLocale'
const tasks = useTaskManager()
const text = pclText
const speed = computed(() =>
  tasks.totalSpeed < 1024
    ? `${tasks.totalSpeed.toFixed(0)} B/s`
    : tasks.totalSpeed < 1048576
      ? `${(tasks.totalSpeed / 1024).toFixed(1)} KiB/s`
      : `${(tasks.totalSpeed / 1048576).toFixed(1)} MiB/s`,
)
const stats = computed(() => [
  { title: 'Speed.Progress.Total', value: `${tasks.totalProgress} %` },
  { title: 'Speed.Progress.Speed', value: speed.value },
  { title: 'Speed.Progress.RemainingFiles', value: String(tasks.totalRemaining) },
  { title: 'Speed.Progress.RemainingThreads', value: '—' },
])
onMounted(() => useSideNavState().setWidth(200))
</script>
<template>
  <aside class="speed-left">
    <div v-for="stat in stats" :key="stat.title" class="stat">
      <span>{{ text(stat.title) }}</span>
      <hr />
      <strong>{{ stat.value }}</strong>
    </div>
  </aside>
</template>
<style scoped>
.speed-left {
  width: 200px;
  flex: 0 0 200px;
  display: flex;
  flex-direction: column;
  justify-content: space-evenly;
  padding: 25px;
}
.stat {
  display: flex;
  flex-direction: column;
  align-items: center;
}
.stat > span {
  color: var(--color-tint);
  font-size: 14px;
}
.stat > strong {
  font-size: 20px;
  font-weight: normal;
}
.stat > hr {
  width: 100%;
  height: 2px;
  border: 0;
  background: var(--color-tint-light);
  margin: 7px 0;
}
</style>
