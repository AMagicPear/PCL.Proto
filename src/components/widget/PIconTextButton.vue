<script setup lang="ts">
import PSvgIcon from './PSvgIcon.vue'

defineProps<{ icon: string; disabled?: boolean }>()
</script>

<template>
  <button type="button" class="icon-text-button" :disabled="disabled">
    <PSvgIcon :name="icon" :size="16" /><slot />
  </button>
</template>

<style scoped>
.icon-text-button {
  display: inline-flex;
  align-items: center;
  justify-content: center;
  gap: 7px;
  height: 27px;
  padding: 0 12px;
  border: 0;
  border-radius: 13.5px;
  background: transparent;
  color: var(--color-text-black);
  font: inherit;
  cursor: pointer;
  transition:
    color 150ms,
    background-color 150ms;
}
.icon-text-button:not(:disabled):is(:hover, :focus-visible) {
  background: var(--color-tint-lighter);
  color: var(--color-tint);
}
.icon-text-button:not(:disabled):active {
  background: var(--half-transparent-blue);
}
.icon-text-button:focus-visible {
  outline: 2px solid var(--color-tint);
  outline-offset: 2px;
}
.icon-text-button:disabled {
  color: var(--color-text-grey);
  cursor: not-allowed;
}
@media (prefers-reduced-motion: reduce) {
  .icon-text-button {
    transition: none;
  }
}
</style>
