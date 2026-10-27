<script setup lang="ts">
import { ref } from 'vue'
import PSvgIcon from './PSvgIcon.vue'

export type Severity = 'info' | 'warning' | 'error'
withDefaults(defineProps<{ severity: Severity; closable?: boolean }>(), {
  severity: 'info',
  closable: false,
})

const visibility = ref(true)
</script>

<template>
  <div class="hint-container" :class="severity" v-if="visibility">
    <div><slot></slot></div>
    <button
      type="button"
      aria-label="关闭提示"
      class="button-animated"
      v-if="closable"
      @click="visibility = false"
    >
      <PSvgIcon name="lucide/x" :size="14" />
    </button>
  </div>
</template>

<style lang="css" scoped>
button.button-animated {
  display: flex;
  align-items: center;
  justify-content: center;
}

.hint-container {
  border-radius: 3px;
  border-left: 3px solid;
  padding: 6px 8px;
  font-size: 12px;
  display: flex;
  align-items: center;
  justify-content: space-between;
  margin-top: 2px;
  margin-bottom: 4px;
}

.theme-blue .hint-container.error {
  background-color: #feeaeb;
  border-left-color: #f31628;
  color: #f31628;
}

.theme-blue .hint-container.warning {
  background-color: #fff4e6;
  border-left-color: #f39c16;
  color: #f39c16;
}

.theme-blue .hint-container.info {
  background-color: var(--lighter-blue);
  border-left-color: var(--tint-blue);
  color: var(--tint-blue);
}

.theme-orange .hint-container.info {
  background-color: #ed931d3d;
  border-left-color: #ff9626;
  color: #ff9626;
}

button.button-animated {
  scale: 0.8;
  color: var(--light-blue);
  transition: 0.4s;
}

button.button-animated:hover {
  color: var(--color-titlebar);
}
</style>

<style scoped>
button.button-animated {
  border: 0;
  background: transparent;
  cursor: pointer;
  padding: 3px;
}
</style>
