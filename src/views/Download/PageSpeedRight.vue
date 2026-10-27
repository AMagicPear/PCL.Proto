<script setup lang="ts">
import PCard from '@/components/widget/PCard.vue'
import TaskItem from '@/components/widget/TaskItem.vue'
import { useTaskManager } from '@/stores/task'
const tasks = useTaskManager()
</script>
<template>
  <article class="subview pcl-page speed-right">
    <PCard v-for="task in tasks.tasks" :key="task.id" :title="task.name"
      ><div class="task-items">
        <TaskItem v-for="item in task.items" :key="item.id" v-bind="item" /></div
    ></PCard>
    <p v-if="!tasks.tasks.length" class="pcl-empty">暂无任务</p>
  </article>
</template>
<style scoped>
.speed-right {
  flex: 1;
  min-width: 0;
  overflow-y: auto;
  padding: 25px;
  display: flex;
  flex-direction: column;
  gap: 15px;
}
.task-items {
  padding: 0 20px 15px;
}
</style>
