<template>
  <button class="pcl-primary-action" type="button"><slot /></button>
</template>

<style scoped>
.pcl-primary-action {
  display: inline-flex;
  align-items: center;
  justify-content: center;
  align-self: center;
  gap: 8px;
  min-width: 120px;
  height: 36px;
  padding: 0 12px;
  border: 0;
  border-radius: 30px;
  background: var(--tint-blue);
  color: var(--vt-c-white);
  font-family: inherit;
  font-size: 14px;
  box-shadow: var(--box-shadow);
  cursor: pointer;
  /* CE MyButton：悬停背景 100/200 ms，按下 80 ms，松开 300 ms（延迟 10 ms）。 */
  transition:
    background-color 200ms linear,
    transform 300ms 10ms cubic-bezier(0.33, 1, 0.68, 1);
}
.pcl-primary-action:hover:not(:disabled) {
  background-color: var(--light-blue);
  transition-duration: 100ms, 300ms;
}
.pcl-primary-action:active:not(:disabled) {
  transform: scale(0.955);
  transition-duration: 200ms, 80ms;
  transition-delay: 0ms, 0ms;
  transition-timing-function: linear, cubic-bezier(0.22, 1, 0.36, 1);
}
.pcl-primary-action:focus-visible {
  outline: 2px solid var(--color-tint);
  outline-offset: 2px;
}
.pcl-primary-action:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}
@media (prefers-reduced-motion: reduce) {
  .pcl-primary-action {
    transition: none;
  }
}
</style>
